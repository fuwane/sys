use extism::CurrentPlugin;

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
        &self, plugin: &mut CurrentPlugin,
        ctx: Context
    ) -> Result<(), Error> {
        plugin.vars.insert(self.key.clone(), to_vec(&ctx)?); Ok(())
    }
}

impl From<String> for ContextManager {
    fn from(channel_id: String) -> Self {
        Self { key: format!("{}c", channel_id) }
    }
}

impl From<u64> for ContextManager {
    fn from(channel_id: u64) -> Self {
        Self { key: format!("{}c", channel_id) }
    }
}