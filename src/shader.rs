extern crate gl;
use gl::*;
use gl::types::GLenum;

pub struct Shader {
    pub shader: GLenum,
}

impl Shader {
    pub unsafe fn new(shader_type: GLenum, shader_path: &str) -> Self {
        let shader = CreateShader(shader_type);
        let source = std::fs::read_to_string(shader_path).expect(format!("Shader File: {shader_path} not found!", shader_path=shader_path).as_str());
        ShaderSource(
            shader,
            1,
            &(source.as_bytes().as_ptr().cast()),
            &(source.len().try_into().unwrap()),
        );
        CompileShader(shader);
        let mut success = 0;
        GetShaderiv(shader, COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
        Self {
            shader: shader,
        }
    }
}
