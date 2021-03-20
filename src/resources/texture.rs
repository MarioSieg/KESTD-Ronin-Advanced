use super::prelude::*;

pub type Texel = u8;

pub struct Texture {
    path: PathBuf,
    width: u16,
    height: u16,
    texels: Box<[Texel]>,
    extent: wgpu::Extent3d,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
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
}

impl ResourceImporteur for Texture {
    type ImportSystem = graphics::GraphicsSystem;

    #[inline]
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn load(system: &Self::ImportSystem, path: PathBuf) -> Arc<Self> {
        use wgpu::*;

        let width: u16 = 256;
        let height: u16 = 256;
        let texels = create_texels(width as _).into_boxed_slice();
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
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        system.drivers.queue.write_texture(
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &texels,
            TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width as u32,
                rows_per_image: 0,
            },
            extent,
        );
        Arc::new(Self {
            path,
            width,
            height,
            texels,
            extent,
            texture,
            view,
        })
    }
}

fn create_texels(size: usize) -> Vec<u8> {
    (0..size * size)
        .flat_map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            std::iter::once(0xFF - (count * 5) as u8)
                .chain(std::iter::once(0xFF - (count * 15) as u8))
                .chain(std::iter::once(0xFF - (count * 50) as u8))
                .chain(std::iter::once(1))
        })
        .collect()
}
