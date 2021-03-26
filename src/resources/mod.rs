pub mod material;
pub mod mesh;
pub mod texture;

use super::systems::SubSystem;
use crate::resources::prelude::PathBuf;
use mesh::Mesh;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use texture::Texture;

pub type ResourceId = u64;

pub trait Resource {
    type ImportSystem: SubSystem;

    fn load(_system: &Self::ImportSystem, raw_data: Vec<u8>) -> Self;
}

pub struct ResourceCache<T: Resource>(HashMap<ResourceId, Arc<T>>);

impl<T: Resource> ResourceCache<T> {
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }

    #[inline]
    pub fn new(table: HashMap<ResourceId, Arc<T>>) -> Self {
        Self(table)
    }

    #[inline]
    pub fn table(&self) -> &HashMap<ResourceId, Arc<T>> {
        &self.0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn contains(&self, other: ResourceId) -> bool {
        self.0.contains_key(&other)
    }

    #[inline]
    pub fn get(&self, other: ResourceId) -> Option<&Arc<T>> {
        self.0.get(&other)
    }

    #[inline]
    pub fn insert(&mut self, k: ResourceId, v: Arc<T>) {
        self.0.insert(k, v);
    }

    pub fn load_imm(&mut self, system: &T::ImportSystem, path: &str) -> Arc<T> {
        self.load(system, PathBuf::from(path))
    }

    pub fn load(&mut self, system: &T::ImportSystem, path: PathBuf) -> Arc<T> {
        let id = {
            let mut hasher = DefaultHasher::new();
            path.hash(&mut hasher);
            hasher.finish()
        };
        // if resource is already loaded, just return the pointer
        if let Some(ptr) = self.get(id) {
            ptr.clone()
        } else {
            // else load the file and insert it:
            let bytes: Vec<u8> = std::fs::read(&path).unwrap_or_else(|_| {
                panic!("Cache import error! Failed to read file: {:?}", path);
            });
            let ptr = Arc::new(T::load(system, bytes));
            self.insert(id, ptr.clone());
            ptr
        }
    }
}

pub struct ResourceManager {
    pub texture_cache: ResourceCache<Texture>,
    pub mesh_cache: ResourceCache<Mesh>,
}

impl ResourceManager {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            texture_cache: ResourceCache::with_capacity(capacity),
            mesh_cache: ResourceCache::with_capacity(capacity),
        }
    }
}

mod prelude {
    pub use super::Resource;
    pub use crate::systems::*;
    pub use std::{path::PathBuf, sync::Arc};
}
