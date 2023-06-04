use std::{ collections::HashMap, sync::Arc };

use extism::{ Context, Manifest };

pub mod service;
pub mod binding;

pub use service::Service;
pub use binding::{ Channel, SharedSpace };


pub struct OpenSpace;


pub struct GlobalSpace {
    pub open: OpenSpace,
    pub shared: Arc<SharedSpace>
}


pub struct Manager<'a> {
    pub shared_space: Arc<SharedSpace>,
    pub services: HashMap<String, Service<'a>>,
    ctx: Context
}


impl<'a> Manager<'a> {
    pub fn new() -> Self {
        Self {
            shared_space: Arc::new(SharedSpace::default()),
            services: HashMap::new(), ctx: Context::new()
        }
    }

    pub fn add_service(&'a mut self, name: String, manifest: &Manifest, wasi: bool) {
        self.services.insert(name.clone(), Service::new(
            name, &self.ctx, self.shared_space.clone(), manifest, wasi
        ));
    }
}