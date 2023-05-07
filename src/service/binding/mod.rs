//! FuwaNe System Service - Binding

use tokio::spawn;
use once_cell::sync::Lazy;

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error,
    Function, ValType
};

pub mod reader;
pub mod channel;

use crate::{ Manager, MANAGERS };
use reader::WasmAudioReader;
pub use channel::Channel;


// ここからイベント関連。


/// 送信する音声チャンネルのID（`u64`）を`i64`型にした数値です。
pub type ChannelIdI64 = i64;


#[derive(Debug)]
pub struct PlayData {
    pub channel_id: u64,
    pub input: Input
}


#[derive(Debug)]
pub enum Event {
    Play(PlayData)
}

impl Event {
    pub fn handle<'a>(self, _manager: &mut Manager<'a>) -> Result<(), Error> {
        Ok(())
    }
}


// ここから橋渡し。


fn make_cnf_text(channel_id: u64) -> String {
    format!("The channel with the ID {} is not currently connected.", channel_id)
}
pub fn play(
    _plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], _user_data: UserData
) -> Result<(), Error> {
    // 再生を行う。（実際の音源は一度で読み込まずちょっとずつ読み込まれる。）
    let manager_id = inputs[0].unwrap_i32() as u32;
    let channel_id = inputs[1].unwrap_i64() as u64;
    let is_stereo = inputs[2].unwrap_i32() > 0;
    let a = &_plugin.vars;
    spawn(async move {
        if let Some(channel) = MANAGERS..await.shared
                .get_mut(&manager_id).unwrap().channels.get_mut(&channel_id) {
            channel.play(Input::float_pcm(
                is_stereo, Reader::Extension(Box::new(
                    WasmAudioReader { manager_id, channel_id }
                ))
            ));
        } else { println!("{}", make_cnf_text(channel_id)); }; // TODO: Log it.
    });
    Ok(())
}


pub fn send_audio_frame(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    outputs: &mut [Val], _user_data: UserData
) -> Result<(), Error> {
    let manager_id = inputs[0].unwrap_i32() as u32;
    let channel_id = inputs[1].unwrap_i64() as u64;
    spawn(async move {
        if let Some(channel) = MANAGERS.write().await
    });
    Ok(())
}


pub static FUNCTIONS: Lazy<Vec<Function>> = Lazy::new(|| vec![
    Function::new("play", [ValType::I64, ValType::I32], [], None, play)
]);
