extern crate gl;
use gl::*;
use gl::types::{GLenum, GLuint};

pub struct ShaderProgram {
    pub vs: GLenum,
    pub fs: GLenum,
    pub program: GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(vs_path: &str, fs_path: &str) -> Self {
        let vs = CreateShader(VERTEX_SHADER);
        let fs = CreateShader(FRAGMENT_SHADER);
        let vs_source = std::fs::read_to_string(vs_path).expect(format!("Shader File: {shader_path} not found!", shader_path=vs_path).as_str());
        ShaderSource(
            vs,
            1,
            &(vs_source.as_bytes().as_ptr().cast()),
            &(vs_source.len().try_into().unwrap()),
        );
        CompileShader(vs);
        let mut success = 0;
        GetShaderiv(vs, COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                vs,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
        let fs_source = std::fs::read_to_string(fs_path).expect(format!("Shader File: {shader_path} not found!", shader_path=fs_path).as_str());
        ShaderSource(
            fs,
            1,
            &(fs_source.as_bytes().as_ptr().cast()),
            &(fs_source.len().try_into().unwrap()),
        );
        CompileShader(fs);
        let mut success = 0;
        GetShaderiv(fs, COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            GetShaderInfoLog(
                fs,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
        let program = CreateProgram();
        AttachShader(program, vs);
        AttachShader(program, fs);
        LinkProgram(program);
        let mut success = 0;
        GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            GetProgramInfoLog(
                program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        UseProgram(program);
        DeleteShader(vs);
        DeleteShader(fs);
        Self {
            vs: vs,
            fs: fs,
            program: program
        }
    }
}