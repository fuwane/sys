//! FuwaNe - Service

use extism::Plugin;

pub mod config;
pub mod binding;

use config::Config;
use binding::{ Event as BindingEvent, WasmBridge };


#[derive(Debug)]
pub enum Event {
    Binding(BindingEvent)
}


pub struct Service<'a> {
    pub config: Config,
    pub plugin: Plugin<'a>,
    pub wasm_bridge: WasmBridge
}