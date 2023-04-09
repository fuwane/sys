//! FuwaNe - Service

use extism::{ Context, Plugin };

pub mod utils;
pub mod config;
pub(crate) mod binding;

use config::Config;
use binding::WASM_FOUNDATION;


pub struct Service<'a> {
    pub config: Config,
    pub plugin: Plugin<'a>
}

impl<'a> Service<'a> {
    pub fn new(ctx: &'a Context, config: Config, data: &[u8]) -> Self {
        Self { plugin: Plugin::new(
            ctx, data, [&WASM_FOUNDATION.play],
            config.wasi
        ).unwrap(), config: config }
    }
}