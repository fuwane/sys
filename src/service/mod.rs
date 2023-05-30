use std::sync::Arc;

use extism::{ Plugin, Manifest, Context  };

use crate::{ SharedSpace, binding::make_functions };


pub struct Service<'a> {
    pub name: String,
    plugin: Plugin<'a>
}

impl<'a> Service<'a> {
    pub fn new(
        name: String, ctx: &'a Context,
        shared_space: Arc<SharedSpace>,
        manifest: &Manifest, wasi: bool
    ) -> Self {
        Self { name: name, plugin: Plugin::new_with_manifest(
            ctx, manifest, make_functions(shared_space), wasi
        ).unwrap() }
    }
}