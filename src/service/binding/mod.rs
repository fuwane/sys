//! FuwaNe Service - Binding

use std::collections::{ HashMap, VecDeque };

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error as ExtismError, Function, ValType
};

use tokio::{ sync::{ Mutex as AioMutex }, spawn };
use once_cell::sync::Lazy;

use crate::{ Event as RootEvent, EVENT_CHANNEL };
use super::Event as ServiceEvent;

pub mod reader;


pub struct PlayData {
    pub channel_id: u64,
    pub input: Input
}


pub enum Event {
    Play(PlayData)
}


pub type AudioBuffer = AioMutex<HashMap<u64, VecDeque<Vec<u8>>>>;
pub static AUDIO_BUFFER: Lazy<AudioBuffer> =
    Lazy::new(|| AioMutex::new(HashMap::new()));

pub fn play(
    _plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], _user_data: UserData
) -> Result<(), ExtismError> {
    // 再生を行う。（実際の音源は一度で読み込まずちょっとずつ読み込まれる。）
    let channel_id = inputs[0].unwrap_i64() as u64;
    spawn(EVENT_CHANNEL.tx.send(RootEvent::Service(ServiceEvent::Binding(
        Event::Play(PlayData {channel_id: channel_id, input: Input::float_pcm(
            inputs[1].unwrap_i32() > 0, Reader::Extension(
                Box::new(reader::WasmAudioReader {
                    channel_id: channel_id
                })
            )
        )})))
    ));
    Ok(())
}


pub fn send_audio_data(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], _user_data: UserData
) -> Result<(), ExtismError> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    if let Some(data) = plugin.vars.get(&channel_id.to_string()) {
        let cloned = data.to_vec();
        spawn(async move {
            let mut buffer = AUDIO_BUFFER.lock().await;
            if !buffer.contains_key(&channel_id) {
                buffer.insert(channel_id, VecDeque::new());
            };
            buffer.get_mut(&channel_id).unwrap().push_back(cloned);
        });
    };
    Ok(())
}



pub struct WasmBridge {
    pub play: Function, pub send_audio_data: Function
}
impl WasmBridge {
    pub fn get_slice(&self) -> [&Function; 2] {
        [&self.play, &self.send_audio_data]
    }
}
pub static BRIDGE: Lazy<WasmBridge> = Lazy::new(|| WasmBridge {
    play: Function::new("play", [ValType::I64, ValType::I32], [], None, play),
    send_audio_data: Function::new(
        "send_audio_data", [ValType::I64],
        [], None, send_audio_data
    )
});