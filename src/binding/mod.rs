use std::{ collections::HashMap, sync::Arc };

use tokio::spawn;
use anyhow::Context;

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error,
    Function, ValType
};

pub mod communication;
pub mod reader;
pub mod channel;

pub use channel::Channel;


/// 送信する音声チャンネルのID（`u64`）を`i64`型にした数値です。
pub type ChannelIdI64 = i64;

#[derive(Default)]
pub struct SharedSpace {
    pub channels: HashMap<u64, Channel>
}

impl SharedSpace {
    pub fn channels(&self) -> &HashMap<u64, Channel> { &self.channels }
}


pub fn get_shared_space(user_data: &UserData) -> &SharedSpace {
    user_data.any().unwrap().downcast_ref().unwrap()
}
pub fn get_mut_shared_space(user_data: &mut UserData) -> &mut SharedSpace {
    user_data.any_mut().unwrap().downcast_mut().unwrap()
}

pub fn get_channel(
    shared_space: &SharedSpace,
    channel_id: u64
) -> Result<&Channel, Error> {
    shared_space.channels.get(&channel_id).context(make_cnf_text(channel_id))
}
pub fn get_mut_channel(
    shared_space: &mut SharedSpace,
    channel_id: u64
) -> Result<&mut Channel, Error> {
    shared_space.channels.get_mut(&channel_id).context(make_cnf_text(channel_id))
}

fn make_cnf_text(channel_id: u64) -> String {
    format!("The channel with the ID {} is not currently connected.", channel_id)
}


/// Play the audio.
/// When this function is executed, audio can be played on the specified channel. This function also returns a track ID to manage the playback of the audio. The track ID is also required if any processing is to be performed on the audio playback.
/// # Notes
/// The audio to be played must be sent using the `send_audio_frame` function.
/// # Inputs
/// - `channel_id: i64`
/// - `is_stereo: i64`
/// # Outputs
/// - `track_id: i64` - It is the offset of the track id in Extism's memory that is u128.
pub fn play(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    outputs: &mut [Val], mut user_data: UserData
) -> Result<(), Error> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    let is_stereo = inputs[1].unwrap_i32() > 0;

    // チャンネルを取得する。
    let channel = get_mut_channel(get_mut_shared_space(&mut user_data), channel_id)?;
    let aampsc = channel.acquire();

    // 再生を開始する。
    // NOTE: 下にある`channel.play`が返すトラックIDはv128だが、返り値をv128の別の表現として`Vec<u8>`としているのは理由がある。
    // 現在、v128の型をRustのexternブロックで使用することができない。
    // Extismはプラグイン側のホスト関数の定義時にexternブロックを使用するため、ここで返り値をV128としてプラグイン側の定義でv128の型を使用すると、前述のことからエラーが発生する。
    // そのため、現在は`Val::V128`ではなく、`Vec<u8>`にしてトラックIDを返す。
    // v128が使えるようになり次第、つまり、次のIssueが解決次第、ここはv128に変更する：https://github.com/rust-lang/rust/issues/27731
    outputs[0] = Val::I64(unsafe { plugin.memory.as_mut() }.unwrap().alloc_bytes(
        channel.play(Input::float_pcm(
            is_stereo, Reader::Extension(Box::new(reader::WasmAudioReader {
                channel_id, receiver: aampsc.0.1.clone()
            }))
        ), &aampsc).to_be_bytes()
    ).unwrap().offset as _);

    Ok(())
}

/// Send audio frame to Discord.
/// # Inputs
/// - `channel_id_i64: i64` - ID of the channel expressed in i64.
/// - `track_id: i64` - A offset of the track id in Extism's memory.
/// - `audio_frame_data: i64` - A offset of the audio frame data in Extism's memory.
pub fn send_audio_frame(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], mut user_data: UserData
) -> Result<(), Error> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    let memory = unsafe { plugin.memory.as_mut() }.unwrap();
    let track_id = u128::from_be_bytes(
        memory.get(inputs[1].unwrap_i64() as usize).unwrap()
            .try_into().unwrap()
    );
    // 準備。
    let channel = get_mut_channel(get_mut_shared_space(&mut user_data), channel_id)?;
    let track = channel.core.tracks.get_mut(&track_id).unwrap();
    // プラグインから来た音声を送る。
    let sender = track.sender.clone();
    let frame = memory.get(inputs[2].unwrap_i64() as usize)
        .expect("No audio frame were passed.").try_into().unwrap();
    spawn(async move { let _ = sender.send(frame).await; });
    Ok(())
}


/// Make Extism's function objects.
pub fn make_functions(shared_space: Arc<SharedSpace>) -> [Function;2] { [
    Function::new(
        "play", [ValType::I64, ValType::I32], [ValType::I64, ValType::I64],
        Some(UserData::new(shared_space.clone())), play
    ),
    Function::new(
        "send_audio_frame", [ValType::I64, ValType::I64], [],
        Some(UserData::new(shared_space)), send_audio_frame
    )
] }