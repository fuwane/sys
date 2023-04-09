//! FuwaNe Service - Binding

use std::sync::Arc;

use songbird::input::{ Input, Reader };

use extism::{
    CurrentPlugin, Val, UserData, Error as ExtismError, Function, ValType
};

use tokio::{ sync::{ mpsc::{ Sender, Receiver, channel }, Mutex }, spawn };
use once_cell::sync::Lazy;

use crate::ChannelId;

pub mod reader;


pub struct PlayData {
    pub channel_id: ChannelId,
    pub input: Input
}


pub enum DataFromService {
    Play(PlayData)
}


pub fn play(
    plugin: &mut CurrentPlugin, inputs: &[Val],
    _outputs: &mut [Val], _user_data: UserData
) -> Result<(), ExtismError> {
    spawn(WASM_FOUNDATION.tx.send(DataFromService::Play(PlayData {
        channel_id: inputs[0].unwrap_i64() as u64, input: Input::float_pcm(
            inputs[1].unwrap_i32() > 0, Reader::Extension(
                Box::new(reader::WasmAudioReader { vars: plugin.vars })
            )
        )
    })));
    Ok(())
}


pub struct WasmEventReceiver {
    pub tx: Sender<DataFromService>,
    pub rx: Arc<Mutex<Receiver<DataFromService>>>,
    pub play: Function
}
pub static WASM_FOUNDATION: Lazy<WasmEventReceiver> = Lazy::new(|| {
    let (tx, rx) = channel(128);
    WasmEventReceiver {
        tx: tx, rx: Arc::new(Mutex::new(rx)),
        play: Function::new("play", [ValType::I64, ValType::I32], [], None, play)
    }
});