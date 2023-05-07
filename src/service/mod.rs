//! FuwaNe - Service

use extism::{ Plugin, Context, Manifest, Error };
use serde_json::to_vec;

use fuwane_foundation::binding::PluginData;

pub mod config;
pub mod binding;

use crate::Manager;
use config::Config;
use binding::{ Event as BindingEvent, FUNCTIONS };


#[derive(Debug)]
pub enum Event {
    Binding(BindingEvent)
}

impl Event {
    pub fn handle<'a>(self, manager: &mut Manager<'a>) -> Result<(), Error> {
        match self {
            Event::Binding(binding_event) => binding_event.handle(manager)
        }
    }
}


pub struct Service<'a> {
    pub id: u32,
    pub config: Config,
    pub plugin: Plugin<'a>,
}

impl<'a> Service<'a> {
    pub fn new(
        id: u32, manager_id: u32,
        ctx: &'a Context, manifest: &Manifest, config: Config
    ) -> Self {
        let mut s = Self {
            id, config, plugin: Plugin::new_with_manifest(
                ctx, manifest, FUNCTIONS.iter(), false).unwrap()
        };
        s.plugin.call("setup", to_vec(
            &PluginData { manager_id, service_id: id }
        ).unwrap()).unwrap();
        s
    }
}
