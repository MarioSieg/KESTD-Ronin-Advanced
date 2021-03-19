use super::prelude::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    pub const fn new(position: [f32; 4], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }

    pub const fn from_integers(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
            tex_coords: [tc[0] as f32, tc[1] as f32],
        }
    }
}

pub type Index = u16;

pub struct Mesh {
    path: PathBuf,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices: Box<[Index]>,
    vertices: Box<[Vertex]>,
}

impl ResourceImporteur for Mesh {
    type ImportSystem = crate::systems::graphics::GraphicsSystem;

    #[inline]
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn load(system: &Self::ImportSystem, path: PathBuf) -> Option<Arc<Self>> {
        use wgpu::util::{BufferInitDescriptor, DeviceExt};
        use wgpu::*;

        let vertices: Box<[Vertex]> = Box::from(CUBE_VERTICES);
        let indices: Box<[Index]> = Box::from(CUBE_INDICES);

        let vertex_buffer = system
            .drivers
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices[..]),
                usage: BufferUsage::VERTEX,
            });

        let index_buffer = system
            .drivers
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&indices[..]),
                usage: BufferUsage::INDEX,
            });

        Some(Arc::new(Self {
            path,
            vertex_buffer,
            index_buffer,
            indices,
            vertices,
        }))
    }
}

impl Mesh {
    #[inline]
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    #[inline]
    pub fn indices(&self) -> &[Index] {
        &self.indices
    }

    #[inline]
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
}

pub const CUBE_VERTICES: [Vertex; 24] = [
    // top (0, 0, 1)
    Vertex::from_integers([-1, -1, 1], [0, 0]),
    Vertex::from_integers([1, -1, 1], [1, 0]),
    Vertex::from_integers([1, 1, 1], [1, 1]),
    Vertex::from_integers([-1, 1, 1], [0, 1]),
    // bottom (0, 0, -1)
    Vertex::from_integers([-1, 1, -1], [1, 0]),
    Vertex::from_integers([1, 1, -1], [0, 0]),
    Vertex::from_integers([1, -1, -1], [0, 1]),
    Vertex::from_integers([-1, -1, -1], [1, 1]),
    // right (1, 0, 0)
    Vertex::from_integers([1, -1, -1], [0, 0]),
    Vertex::from_integers([1, 1, -1], [1, 0]),
    Vertex::from_integers([1, 1, 1], [1, 1]),
    Vertex::from_integers([1, -1, 1], [0, 1]),
    // left (-1, 0, 0)
    Vertex::from_integers([-1, -1, 1], [1, 0]),
    Vertex::from_integers([-1, 1, 1], [0, 0]),
    Vertex::from_integers([-1, 1, -1], [0, 1]),
    Vertex::from_integers([-1, -1, -1], [1, 1]),
    // front (0, 1, 0)
    Vertex::from_integers([1, 1, -1], [1, 0]),
    Vertex::from_integers([-1, 1, -1], [0, 0]),
    Vertex::from_integers([-1, 1, 1], [0, 1]),
    Vertex::from_integers([1, 1, 1], [1, 1]),
    // back (0, -1, 0)
    Vertex::from_integers([1, -1, 1], [0, 0]),
    Vertex::from_integers([-1, -1, 1], [1, 0]),
    Vertex::from_integers([-1, -1, -1], [1, 1]),
    Vertex::from_integers([1, -1, -1], [0, 1]),
];

pub const CUBE_INDICES: [u16; 36] = [
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

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
