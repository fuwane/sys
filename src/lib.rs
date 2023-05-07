//! FuwaNe System

use std::{ collections::{ HashMap, BTreeSet, VecDeque }, hash::{ Hash, Hasher } };

use tokio::{ sync::RwLock as AioRwLock, runtime::Handle };
use pollster::FutureExt as _;
use once_cell::sync::Lazy;

use extism::{ Context, Manifest, Error };

use songbird::Call;

use fuwane_foundation::communication::create_lazy_channel;

pub mod utils;
pub mod event;
pub mod types;
pub mod client;
pub mod service;

pub(crate) use event::EventChannel;
use service::{ binding::Channel, config::Config, Service };


#[derive(Default)]
pub struct IdManager(AioRwLock<BTreeSet<u32>>);
impl IdManager {
    pub async fn acquire_async(&mut self) -> u32 {
        let mut ids = self.0.write().await;
        let id = {
            let mut next_iter = ids.iter();
            if let Some(first) = next_iter.next() {
                if *first > 0 { return first - 1; };
            } else { return 0; };
            let mut last = 0;
            for zipped in ids.iter().zip(next_iter) {
                if zipped.1 - zipped.0 > 1 {
                    return zipped.1 + 1;
                };
                last = *zipped.1;
            };
            if last == 0 { last } else { last + 1 }
        };
        ids.insert(id);
        id
    }

    pub fn acquire(&mut self) -> u32 {
        self.acquire_async().block_on()
    }

    pub async fn release_async(&mut self, id: &u32) {
        self.0.write().await.remove(id);
    }

    pub fn release(&mut self, id: &u32) {
        self.release_async(id).block_on();
    }
}


pub struct Shared<'a> {
    pub channels: HashMap<u64, Channel>,
    pub events: EventChannel<'a>,
}

impl<'a> Shared<'a> {
    pub fn calls(&self) -> &HashMap<u64, Channel> { &self.channels }

    pub fn add_call(&mut self, channel_id: u64, call: Call) {
        self.channels.insert(channel_id, Channel::new(channel_id, call));
    }

    pub fn remove_call(&mut self, id: &u64) -> Option<Channel> {
        self.channels.remove(id)
    }
}


#[derive(Default)]
pub struct ManagerPool<'a> {
    ids: IdManager,
    shared: HashMap<u32, Shared<'a>>
}

pub static MANAGERS: Lazy<AioRwLock<ManagerPool>> = Lazy::new(
    || AioRwLock::new(ManagerPool::default()));


pub struct Manager<'a> {
    pub(crate) id: u32,
    pub ctx: Context,
    service_id_manager: IdManager,
    services: HashMap<u32, Service<'a>>,
}

impl<'a> PartialEq for Manager<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<'a> Eq for Manager<'a> {}

impl<'a> Drop for Manager<'a> {
    fn drop(&mut self) {
        if let Ok(hwnd) = Handle::try_current() {
            let id = self.id;
            hwnd.spawn(async move {
                let mut managers = MANAGERS.write().await;
                managers.ids.release_async(&id).await;
                managers.shared.remove(&id);
            });
        };
        // TODO: この場合、適切に破棄できないことに関して警告を出す。
        let mut managers = MANAGERS.blocking_write();
        managers.ids.release(&self.id);
        managers.shared.remove(&self.id);
    }
}

impl<'a> Hash for Manager<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a> Manager<'a> {
    pub async fn new_async() -> Manager<'a> {
        let mut managers = MANAGERS.write().await;
        let id = managers.ids.acquire_async().await;
        managers.shared.insert(id, Shared {
            channels: HashMap::new(),
            events: create_lazy_channel()
        });
        Self {
            id, ctx: Context::new(),
            service_id_manager: IdManager::default(),
            services: HashMap::new()
        }
    }

    pub fn new() -> Manager<'a> {
        Self::new_async().block_on()
    }

    pub fn id(&self) -> &u32 { &self.id }

    pub fn services(&'a self) -> &HashMap<u32, Service> { &self.services }

    pub async fn add_service_async(&'a mut self, manifest: &Manifest, config: Config) {
        let id = self.service_id_manager.acquire_async().await;
        let service = Service::new(id, self.id, &self.ctx, manifest, config);
        self.services.insert(id, service);
    }

    pub fn add_service(&'a mut self, config: Config, manifest: &Manifest) {
        self.add_service_async(manifest, config).block_on();
    }

    pub fn remove_service(&'a mut self, id: u32) -> Option<Service> {
        self.services.remove(&id)
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let rx = MANAGERS.write().await.shared
            .get(&self.id).unwrap().events.rx.clone();
        loop {
            if let Some(event) = rx.lock().await.recv().await {
                if let Err(e) = event.handle(self) {
                    if cfg!(debug_assertions) {
                        return Err(e);
                    } else {
                        // TODO: ここをログ出力に変える。
                        println!("WARNING: {}", e.to_string());
                    };
                };
            };
        };
    }
}
