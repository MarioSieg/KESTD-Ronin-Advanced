use super::prelude::*;
use std::num::NonZeroU8;

pub type Texel = u8;

pub struct Texture {
    path: PathBuf,
    width: u32,
    height: u32,
    texels: Box<[Texel]>,
    extent: wgpu::Extent3d,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
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
    type MetaData = PathBuf;

    #[inline]
    fn meta_data(&self) -> &Self::MetaData {
        &self.path
    }

    fn load(system: &Self::ImportSystem, path: Self::MetaData) -> Arc<Self> {
        use image::io::Reader as ImageReader;
        use wgpu::*;

        let image = ImageReader::open(&path)
            .unwrap()
            .decode()
            .unwrap()
            .flipv()
            .into_rgba8();
        let width = image.width();
        let height = image.height();
        let texels = image.into_raw().into_boxed_slice();

        let mip_level_count = if width == height {
            (width.max(height) as f64).log2().floor() as u32
        } else {
            1
        };

        let extent = Extent3d {
            width,
            height,
            depth: 1,
        };

        let format = TextureFormat::Rgba8UnormSrgb;

        let texture = system.drivers.device.create_texture(&TextureDescriptor {
            label: None,
            size: extent,
            mip_level_count,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsage::SAMPLED | TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_DST,
        });

        system.drivers.queue.write_texture(
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &texels,
            TextureDataLayout {
                offset: 0,
                bytes_per_row: (texels.len() as f64 / extent.height as f64) as u32,
                rows_per_image: 0,
            },
            extent,
        );

        let mut encoder = system
            .drivers
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        system
            .drivers
            .generate_mipmaps(&mut encoder, &texture, format, mip_level_count);
        system.drivers.queue.submit(Some(encoder.finish()));

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = system.drivers.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            anisotropy_clamp: NonZeroU8::new(16),
            ..std::default::Default::default()
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
