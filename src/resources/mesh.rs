use super::prelude::*;
use bytemuck::{Pod, Zeroable};
use humantime::Duration;
use log::info;
use std::io::{BufReader, Cursor};
use std::time::Instant;

#[repr(C)]
#[derive(Clone, Copy, Default)]
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
    indices: Box<[Index]>,
    vertices: Box<[Vertex]>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
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

impl Resource for Mesh {
    type ImportSystem = graphics::GraphicsSystem;

    fn load(system: &Self::ImportSystem, raw_data: Vec<u8>) -> Self {
        use obj::{load_obj, Obj, TexturedVertex};
        use rayon::iter::*;
        use wgpu::util::{BufferInitDescriptor, DeviceExt};
        use wgpu::*;

        let input = BufReader::new(Cursor::new(raw_data));
        let mesh: Obj<TexturedVertex> = load_obj(input).unwrap();

        let clock = Instant::now();

        let mut vertices: Vec<Vertex> = mesh
            .vertices
            .par_iter()
            .map(|v: &TexturedVertex| Vertex {
                position: [v.position[0], v.position[1], v.position[2], 1.0],
                tex_coords: [v.texture[0], v.texture[1]],
            })
            .collect();

        // convert to 32-bit indices:
        let mut indices: Vec<u32> = mesh.indices.par_iter().map(|x| *x as u32).collect();

        // optimize mesh:
        meshopt::optimize_vertex_cache_in_place(&indices[..], vertices.len());
        meshopt::optimize_vertex_fetch_in_place(&mut indices[..], &mut vertices[..]);

        // convert back to 16-bit indices
        let indices: Vec<Index> = indices.par_iter().map(|x| *x as Index).collect();

        // convert to boxed slices:
        let vertices = vertices.into_boxed_slice();
        let indices = indices.into_boxed_slice();

        info!(
            "Mesh optimization took: {}",
            Duration::from(clock.elapsed())
        );

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

        Self {
            indices,
            vertices,
            vertex_buffer,
            index_buffer,
        }
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
