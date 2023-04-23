//! FuwaNe Utils - Event

use std::sync::Arc;

use tokio::sync::{ mpsc::{ Sender, Receiver, channel }, Mutex as AioMutex };
use once_cell::sync::Lazy;

use super::service::Event as ServiceEvent;


pub enum Event {
    Service(ServiceEvent)
}


pub struct EventChannel {
    pub tx: Sender<Event>,
    pub rx: Arc<AioMutex<Receiver<Event>>>
}


pub static EVENT_CHANNEL: Lazy<EventChannel> = Lazy::new(|| {
    let (tx, rx) = channel(128); EventChannel {
        tx: tx, rx: Arc::new(AioMutex::new(rx)),
    }
});