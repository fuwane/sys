//! FuwaNe System

use std::collections::HashMap;

use anyhow::{ Context as AHContext, Result as AHResult };
use extism::{ Context, Plugin };

use songbird::Call;

pub mod utils;
pub mod event;
pub mod types;
pub mod client;
pub mod service;

pub(crate) use event::{ Event, EVENT_CHANNEL, InputData };
use service::{
    binding::{ Event as BindingEvent, WasmBridge },
    config::Config, { Service, Event as ServiceEvent }
};


pub struct Manager<'a> {
    pub ctx: Context,
    pub calls: HashMap<u64, Call>,
    services: HashMap<i32, Service<'a>>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Manager<'a> {
        Self {
            ctx: Context::new(), calls: HashMap::new(),
            services: HashMap::new()
        }
    }

    pub fn services(&self) -> &HashMap<i32, Service> { &self.services }

    pub fn add_service(&'a mut self, config: Config, data: &[u8]) {
        let wasm_bridge = WasmBridge::new();
        let service = Service {
            plugin: Plugin::new(&self.ctx, data,[], false).unwrap(),
            config: config, wasm_bridge: wasm_bridge
        };
        let key = service.plugin.as_i32();
        self.services.insert(key, service);
        // 関数を設定。
        let s = self.services.get_mut(&key).unwrap();
        s.wasm_bridge.plugin_id.set(s.plugin.as_i32()).unwrap();
        let functions = s.wasm_bridge.get_slice();
        let wasi = s.config.wasi;
        s.plugin.update(data, functions, wasi).unwrap();
    }

    pub fn remove_service(&mut self, id: i32) -> Option<Service> {
        self.services.remove(&id)
    }

    const RAW_DUMMY: [u8;8] = [0;8];
    async fn handle_event<'b>(&'b mut self, event: Event<'b>) -> AHResult<()> {
        match event {
            Event::Service(service_event) => match service_event {
                ServiceEvent::Binding(binding_event) => match binding_event{
                    BindingEvent::Play(data) => {
                        self.calls.get_mut(&data.channel_id).with_context(|| format!(
                            "No audio connection is made to the channel with ID {}.",
                            data.channel_id
                        ))?.play_source(data.input);
                        Ok(())
                    }
                }
            },
            Event::CallFunction(ctx) => {
                if let Some(s) = self.services.get_mut(&ctx.plugin_id) {
                    let temp = if let InputData::Usize(v) = ctx.input
                        { v.to_ne_bytes() } else { Self::RAW_DUMMY };
                    s.plugin.call(ctx.name, match ctx.input {
                        InputData::Raw(r) => r,
                        InputData::Usize(_) => &temp
                    }).with_context(|| format!(
                        "Failed to call function {} in plugin {}.",
                        ctx.name, ctx.plugin_id
                    ))?;
                };
                Ok(())
            }
        }
    }

    pub async fn start(&mut self) -> AHResult<()> {
        let rx = EVENT_CHANNEL.rx.clone();
        loop {
            if let Err(e) = self.handle_event(
                rx.lock().await.recv().await.unwrap()
            ).await {
                if cfg!(debug_assertions) {
                    return Err(e);
                } else {
                    // TODO: ここをログ出力に変える。
                    println!("WARNING: {}", e.to_string());
                };
            };
        };
    }
}
