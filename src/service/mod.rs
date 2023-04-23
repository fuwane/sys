//! FuwaNe - Service

use extism::{ Context, Plugin };

pub mod config;
pub(crate) mod binding;

use config::Config;
use binding::{ BRIDGE, Event as BindingEvent };


pub enum Event {
    Binding(BindingEvent)
}


pub struct Service<'a> {
    pub config: Config,
    pub plugin: Plugin<'a>
}

impl<'a> Service<'a> {
    pub fn new(ctx: &'a Context, config: Config, data: &[u8]) -> Self {
        Self { plugin: Plugin::new(
            ctx, data, BRIDGE.get_slice(),
            config.wasi
        ).unwrap(), config: config }
    }
}