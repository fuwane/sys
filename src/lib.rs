//! FuwaNe System

use std::collections::HashMap;

use extism::Context;
use extism_runtime::PluginIndex;

use songbird::Call;

pub mod utils;
pub mod event;
pub mod types;
pub mod client;
pub mod service;

pub(crate) use event::{ Event, EVENT_CHANNEL };
use service::{ binding::Event as BindingEvent, { Service, Event as ServiceEvent } };


pub struct Manager<'a> {
    pub ctx: Context,
    pub calls: HashMap<u64, Call>,
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
        let rx = EVENT_CHANNEL.rx.clone();
        loop {
            let data = rx.lock().await.recv().await.unwrap();
            match data {
                Event::Service(service_event) => match service_event {
                    ServiceEvent::Binding(binding_event) => match binding_event{
                        BindingEvent::Play(data) => {
                            self.calls.get_mut(&data.channel_id)
                                .unwrap().play_source(data.input);
                        }
                    }
                }
            };
        };
    }
}