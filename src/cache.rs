use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::Tileset;

pub type ResourcePath = Path;
pub type ResourcePathBuf = PathBuf;

pub trait ResourceCache {
    fn get_tileset(&self, path: impl AsRef<ResourcePath>) -> Option<Arc<Tileset>>;
    fn get_or_try_insert_tileset_with<F, E>(
        &mut self,
        path: ResourcePathBuf,
        f: F,
    ) -> Result<Arc<Tileset>, E>
    where
        F: FnOnce() -> Result<Tileset, E>;
}

/// A cache that identifies resources by their path in the user's filesystem.
pub struct FilesystemResourceCache {
    tilesets: HashMap<ResourcePathBuf, Arc<Tileset>>,
}

impl FilesystemResourceCache {
    pub fn new() -> Self {
        Self {
            tilesets: HashMap::new(),
        }
    }
}

impl ResourceCache for FilesystemResourceCache {
    fn get_tileset(&self, path: impl AsRef<ResourcePath>) -> Option<Arc<Tileset>> {
        self.tilesets.get(path.as_ref()).map(Clone::clone)
    }

    fn get_or_try_insert_tileset_with<F, E>(
        &mut self,
        path: ResourcePathBuf,
        f: F,
    ) -> Result<Arc<Tileset>, E>
    where
        F: FnOnce() -> Result<Tileset, E>,
    {
        Ok(match self.tilesets.entry(path) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => v.insert(Arc::new(f()?)),
        }
        .clone())
    }
}
