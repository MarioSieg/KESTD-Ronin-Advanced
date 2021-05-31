use super::drivers::Drivers;
use log::info;
use shaderc::{CompilationArtifact, CompileOptions, ShaderKind};
use std::fs;
use std::path::PathBuf;

pub const SHADER_ENTRY: &str = "main";
pub const VS_ID: &str = "vert";
pub const FS_ID: &str = "frag";

pub fn compile_to_bytecode(
    drivers: &mut Drivers,
    path: PathBuf,
    shader_type: ShaderKind,
) -> CompilationArtifact {
    let source = fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!("Failed to load shader source file: {:?}", path);
    });
    let sh_opt = &CompileOptions::new().expect("Failed to create compile options!");
    info!("Compiling shader: {:?}", path);
    drivers
        .shader_compiler
        .compile_into_spirv(
            source.as_str(),
            shader_type,
            path.as_os_str().to_str().unwrap_or_else(|| {
                panic!("Failed to convert path to string: {:?}", path);
            }),
            "main",
            Some(sh_opt),
        )
        .unwrap_or_else(|_| {
            panic!("Failed to compile shader source file: {:?}", path);
        })
}
