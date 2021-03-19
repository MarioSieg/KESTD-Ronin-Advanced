use super::systems::System;
use std::path::PathBuf;
use std::sync::Arc;

pub trait Resource {
    type ImportSystem: System;
    type MetaData;

    fn load(_system: &Self::ImportSystem, _file: PathBuf) -> Option<Arc<Self>>;
    fn path(&self) -> &PathBuf;
    fn meta(&self) -> &Self::MetaData;
}
