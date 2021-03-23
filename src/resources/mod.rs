pub mod material;
pub mod mesh;
pub mod texture;

use super::systems::SubSystem;
use std::sync::Arc;

pub trait ResourceImporteur {
    type ImportSystem: SubSystem;
    type MetaData;

    fn meta_data(&self) -> &Self::MetaData;

    fn load(_system: &Self::ImportSystem, _data: Self::MetaData) -> Arc<Self>;
}

mod prelude {
    pub use super::ResourceImporteur;
    pub use crate::systems::*;
    pub use std::{path::PathBuf, sync::Arc};
}
