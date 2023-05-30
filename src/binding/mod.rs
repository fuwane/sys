use std::{ collections::VecDeque, sync::Arc };

use anyhow::Context;

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error,
    Function, ValType
};

use fuwane_foundation::constants::STEREO_FRAME_BYTE_SIZE;

pub mod communication;
pub mod reader;

pub use crate::{ SharedSpace, Channel };


/// 送信する音声チャンネルのID（`u64`）を`i64`型にした数値です。
pub type ChannelIdI64 = i64;


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


pub type Sink = Arc<VecDeque<Vec<u8>>>;
pub fn play(
    _plugin: &mut CurrentPlugin, inputs: &[Val],
    outputs: &mut [Val], mut user_data: UserData
) -> Result<(), Error> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    let is_stereo = inputs[1].unwrap_i32() > 0;
    // チャンネルを取得する。
    let sink = Arc::new(VecDeque::new());
    // 再生を開始する。
    let channel = get_mut_channel(get_mut_shared_space(&mut user_data), channel_id)?;
    outputs[0] = Val::V128(channel.play(Input::float_pcm(
        is_stereo, Reader::Extension(Box::new(
            reader::WasmAudioReader { channel_id, sink: sink.clone() }
        ))
    ), sink));
    Ok(())
}

pub fn send_audio_frame(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], mut user_data: UserData
) -> Result<(), Error> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    let track_id = inputs[1].unwrap_v128();
    let channel = get_mut_channel(get_mut_shared_space(&mut user_data), channel_id)?;
    let track = channel.core.tracks.get_mut(&track_id).unwrap();
    track.sink.push_back(plugin.vars.remove(&track.buffer_id).unwrap());
    Ok(())
}


pub fn make_functions(shared_space: Arc<SharedSpace>) -> [Function;1] { [
    Function::new(
        "play", [ValType::I64, ValType::I32], [ValType::V128],
        Some(UserData::new(shared_space)), play
    ),
] }