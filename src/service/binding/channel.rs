//! FuwaNe System - Channel

use extism::CurrentPlugin;
use songbird::{ Call, input::Input, tracks::TrackHandle };

use anyhow::Error;
use serde_json::{ from_slice, to_vec };

pub use fuwane_foundation::binding::Context;


pub struct ContextManager { pub(crate) key: String }

impl ContextManager {
    pub fn get(&self, plugin: &CurrentPlugin) -> Context {
        if let Some(raw) = plugin.vars.get(&self.key) {
            from_slice(raw.as_slice()).unwrap()
        } else { Context::default() }
    }

    pub fn set(
        &self, plugin: &mut CurrentPlugin, ctx: Context
    ) -> Result<(), Error> {
        plugin.vars.insert(self.key.clone(), to_vec(&ctx)?); Ok(())
    }
}


pub struct BufferManager { pub(crate) key: String }

impl BufferManager {
    pub fn get<'a>(&self, plugin: &'a CurrentPlugin) -> Option<&'a Vec<u8>> {
        plugin.vars.get(&self.key)
    }

    pub fn set(&self, plugin: &mut CurrentPlugin, data: Vec<u8>) {
        plugin.vars.insert(self.key.clone(), data);
    }
}


pub struct Channel {
    pub(crate) call: Call,
    pub id: u64, pub id_i64: i64,
    pub ctx: ContextManager,
    pub buffer: BufferManager,
    pub tracks: Vec<TrackHandle>
}

impl Channel {
    pub fn new(id: u64, call: Call) -> Self{
        let id_string = id.to_string();
        Self {
            call: call, id: id, id_i64: id as _,
            ctx: ContextManager { key: format!("{}c", id_string) },
            buffer: BufferManager { key: format!("{}b", id_string) },
            tracks: Vec::new()
        }
    }

    pub fn play(&mut self, source: Input) {
        self.tracks.push(self.call.play_source(source));
    }
}
