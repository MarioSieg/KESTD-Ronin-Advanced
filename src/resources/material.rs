use super::prelude::*;
use super::texture::Texture;

pub enum MaterialProperties {
    Lambert { path: PathBuf, albedo: Arc<Texture> },
}

pub struct Material {
    pub properties: MaterialProperties,
}
