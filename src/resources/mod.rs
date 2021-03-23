pub mod material;
pub mod mesh;
pub mod texture;

use super::systems::SubSystem;
use std::path::PathBuf;
use std::sync::Arc;

pub trait ResourceImporteur {
    type ImportSystem: SubSystem;

    fn path(&self) -> &PathBuf;

    fn load(_system: &Self::ImportSystem, _path: PathBuf) -> Arc<Self>;
}

mod prelude {
    pub use super::ResourceImporteur;
    pub use crate::systems::*;
    pub use std::{path::PathBuf, sync::Arc};
}
