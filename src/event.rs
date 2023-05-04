//! FuwaNe Utils - Event

use once_cell::sync::Lazy;

use extism::UserData;

pub use fuwane_foundation::communication::Channel as EventChannel;
use fuwane_foundation::communication::create_lazy_channel;

use super::service::Event as ServiceEvent;


#[derive(Debug)]
pub enum InputData<'a> {
    Raw(&'a [u8]),
    Usize(usize)
}

#[derive(Debug)]
pub struct CallContext<'a> {
    pub plugin_id: i32,
    pub name: &'a str,
    pub input: InputData<'a>
}

impl<'a> CallContext<'a> {
    pub fn from_user_data(
        user_data: UserData,
        name: &'a str,
        input: InputData<'a>
    ) -> Option<Self> {
        if let Some(any) = user_data.any() {
            Some(Self {
                plugin_id: *any.downcast_ref().unwrap(),
                name: name.as_ref(), input: input
            })
        } else { None }
    }
}


#[derive(Debug)]
pub enum Event<'a> {
    Service(ServiceEvent),
    CallFunction(CallContext<'a>)
}


pub static EVENT_CHANNEL: Lazy<EventChannel<Event>> = Lazy::new(create_lazy_channel);