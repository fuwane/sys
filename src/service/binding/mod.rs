//! FuwaNe Service - Binding

use std::collections::{ HashMap, VecDeque };

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error as ExtismError, Function, ValType
};

use tokio::{ sync::{ Mutex as AioMutex }, spawn };
use once_cell::sync::{ Lazy, OnceCell };

use crate::{ EVENT_CHANNEL, Event as RootEvent };
use super::Event as ServiceEvent;

pub mod reader;


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


#[derive(Default)]
pub struct Sink {
    pub buffer: VecDeque<Vec<u8>>,
    pub length: usize
}

pub type Sinks = AioMutex<HashMap<u64, Sink>>;
pub static SINKS: Lazy<Sinks> = Lazy::new(|| AioMutex::new(HashMap::new()));

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


/// 音声データを送信するためのExtismプラグイン向けの関数です。
/// 送信するデータ（`Vec<u8>`）は、この関数を実行する前にExtismの変数にてチャンネルIDの文字列をキーとし、事前に配置をしてください。
/// またプラグイン側は、この関数に`ChannelIdI64`を渡す必要があります。
pub fn send_audio_data(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], _user_data: UserData
) -> Result<(), ExtismError> {
    let channel_id = inputs[0].unwrap_i64() as u64;
    if let Some(data) = plugin.vars.get(&channel_id.to_string()) {
        let cloned = data.to_vec();
        spawn(async move {
            let mut sinks = SINKS.lock().await;
            if !sinks.contains_key(&channel_id) {
                sinks.insert(channel_id, Sink::default());
            };
            if let Some(sink) = sinks.get_mut(&channel_id) {
                sink.buffer.push_back(cloned);
            };
        });
    };
    Ok(())
}


pub struct WasmBridge {
    pub plugin_id: OnceCell<i32>,
    pub play: Function,
    pub send_audio_data: Function
}
impl WasmBridge {
    pub fn new() -> Self {
        let plugin_id = OnceCell::new();
        Self {
            plugin_id: plugin_id.clone(),
            play: Function::new(
                "play", [ValType::I64, ValType::I32],
                [], Some(UserData::new(plugin_id.clone())), play
            ),
            send_audio_data: Function::new(
                "send_audio_data", [ValType::I64],
                [], Some(UserData::new(plugin_id)), send_audio_data
            )
        }
    }

    pub fn get_slice(&self) -> [&Function; 2] {
        [&self.play, &self.send_audio_data]
    }
}