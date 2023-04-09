//! FuwaNe System

use std::collections::HashMap;

use extism::Context;
use extism_runtime::PluginIndex;

use songbird::Call;

pub mod types;
pub mod client;
pub mod service;

use service::{ Service, binding::{ WASM_FOUNDATION, DataFromService } };


pub type ChannelId = u64;


pub struct Manager<'a> {
    pub ctx: Context,
    pub calls: HashMap<ChannelId, Call>,
    services: HashMap<PluginIndex, Service<'a>>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Manager<'a> {
        Self {
            ctx: Context::new(), calls: HashMap::new(),
            services: HashMap::new()
        }
    }

    pub fn services(&self) -> &HashMap<PluginIndex, Service> { &self.services }

    pub async fn start(&mut self) {
        let rx = WASM_FOUNDATION.rx.clone();
        loop {
            let data = rx.lock().await.recv().await.unwrap();
            match data {
                DataFromService::Play(data) => {
                    self.calls.get_mut(&data.channel_id)
                        .unwrap().play_source(data.input);
                }
            };
        };
    }
}