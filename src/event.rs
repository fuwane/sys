//! FuwaNe Utils - Event

use std::sync::Arc;

use tokio::sync::{ mpsc::{ Sender, Receiver, channel }, Mutex as AioMutex };
use once_cell::sync::Lazy;

use extism::UserData;

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


pub struct EventChannel<'a> {
    pub tx: Sender<Event<'a>>,
    pub rx: Arc<AioMutex<Receiver<Event<'a>>>>
}


pub static EVENT_CHANNEL: Lazy<EventChannel> = Lazy::new(|| {
    let (tx, rx) = channel(128); EventChannel {
        tx: tx, rx: Arc::new(AioMutex::new(rx))
    }
});