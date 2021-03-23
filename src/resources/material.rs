use super::prelude::*;
use super::texture::Texture;

pub enum Material {
    Lambert { path: PathBuf, albedo: Arc<Texture> },
}
