use super::prelude::*;
use image::GenericImageView;

pub type Texel = u8;

pub struct Texture {
    path: PathBuf,
    width: u16,
    height: u16,
    texels: Box<[Texel]>,
    extent: wgpu::Extent3d,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[inline]
    pub fn textels(&self) -> &[Texel] {
        &self.texels
    }

    #[inline]
    pub fn extend(&self) -> &wgpu::Extent3d {
        &self.extent
    }

    #[inline]
    pub fn buffer(&self) -> &wgpu::Texture {
        &self.texture
    }

    #[inline]
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    #[inline]
    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

impl ResourceImporteur for Texture {
    type ImportSystem = graphics::GraphicsSystem;

    #[inline]
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn load(system: &Self::ImportSystem, path: PathBuf) -> Arc<Self> {
        use image::io::Reader as ImageReader;
        use wgpu::*;

        let image = ImageReader::open(&path).unwrap().decode().unwrap();
        let width = image.width() as u16;
        let height = image.height() as u16;
        let texels = image.into_bytes().into_boxed_slice();

        let extent = Extent3d {
            width: width as u32,
            height: height as u32,
            depth: 1,
        };
        let texture = system.drivers.device.create_texture(&TextureDescriptor {
            label: None,
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        system.drivers.queue.write_texture(
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &texels,
            TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width as u32,
                rows_per_image: 0,
            },
            extent,
        );
        let sampler = system.drivers.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        Arc::new(Self {
            path,
            width,
            height,
            texels,
            extent,
            texture,
            view,
            sampler,
        })
    }
}
