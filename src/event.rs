//! FuwaNe Utils - Event

use anyhow::Context as AHContext;

use extism::Error;

pub use fuwane_foundation::communication::Channel as OriginalEventChannel;

use super::{ Manager, service::Event as ServiceEvent };


/// The types for `FunctionContext`.
#[derive(Debug)]
pub enum InputData<'a> {
    Raw(&'a [u8]),
    Usize(usize)
}

/// The data struct to call a plugin function.
#[derive(Debug)]
pub struct FunctionContext<'a> {
    pub service_id: u32,
    pub name: &'a str,
    pub input: InputData<'a>
}


/// The events.
#[derive(Debug)]
pub enum Event<'a> {
    Service(ServiceEvent),
    CallFunction(FunctionContext<'a>)
}


const RAW_USIZE_DUMMY: [u8;8] = [0;8];
impl<'a> Event<'a> {
    pub fn handle<'b>(self, manager: &mut Manager<'b>) -> Result<(), Error> {
        match self {
            Event::Service(service_event) => service_event.handle(manager),
            Event::CallFunction(ctx) => {
                if let Some(s) = manager.services.get_mut(&ctx.service_id) {
                    let temp = if let InputData::Usize(v) = ctx.input
                        { v.to_ne_bytes() } else { RAW_USIZE_DUMMY };
                    s.plugin.call(
                        ctx.name, match ctx.input {
                            InputData::Raw(r) => r,
                            InputData::Usize(_) => &temp
                        }
                    ).with_context(|| format!(
                        "Failed to call function {} in service {}.",
                        ctx.name, ctx.service_id
                    ))?;
                };
                Ok(())
            }
        }
    }
}


/// It is container of channel for functions to be called from plugin.
/// It can be used to dispatch event to somewhere.
pub type EventChannel<'a> = OriginalEventChannel<Event<'a>>;
