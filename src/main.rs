extern crate glfw;
extern crate gl;
extern crate glm;
extern crate num_traits;
use crate::num_traits::One;
use glfw::{Action, Context, Key};
use num_traits::ToPrimitive;
use std::{f32::consts::PI, process::CommandEnvs};
pub mod camera; // camera stuff
pub mod meshloader; 
pub mod player; 


const ORIGIN: glm::Vector3<f32> = glm::Vector3{ x: 0.0, y: 0.0, z: 0.0 };
const MOVEMENT_DELTA: f32 = 0.005;
const CAMERA_DELTA: f32 = 0.1;

pub fn main() {
    let mut sphere = meshloader::Mesh{vertices: Vec::new()};
    sphere.load("assets/mesh/cube.stl");
    let mut player = player::Player{mesh: sphere, pos: ORIGIN, vec: ORIGIN};
    let mut vertices = Vec::new();
    for vertex in &player.mesh.vertices {
        vertices.extend_from_slice(&mut vertex.as_array().as_slice());
    }

    let mut camera = camera::Camera {
        eye: glm::vec3(0.0, 1.0, 3.0),
        center: glm::vec3(0.0, 0.5, 0.0),
        up: glm::vec3(0.0,PI/3.0,-0.5),
        fov: PI/3.0,
        aspect: 1.0,
        near: 0.1,
        far: 100.0,
    };
    //keys
    let mut keystates = [0,0,0,0,0,0,0,0];
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(800, 800, "Se-Phere!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        
        let t_mat = glm::ext::translate(&glm::Matrix4::<f32>::one(), player.pos);

        unsafe {
            let mut vao = 0u32;
            let mut vbo = 0u32;
            gl::load_with(|f_name| window.get_proc_address(f_name));
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao,0);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo,0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW);
    
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3*std::mem::size_of::<f32>()).try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            const VERT_SHADER: &str = r#"#version 330 core
              layout (location = 0) in vec3 aPos;
              uniform mat4 pv;
              uniform mat4 model;
              void main() {
                gl_Position = pv * model * vec4(aPos, 1.0);
              }
            "#;
            gl::ShaderSource(
                vs,
                1,
                &(VERT_SHADER.as_bytes().as_ptr().cast()),
                &(VERT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(vs);
            let mut success = 0;
            gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
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
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            const FRAG_SHADER: &str = r#"#version 330 core
              out vec4 final_color;

              void main() {
                final_color = vec4(1.0, 0.5, 0.2, 1.0);
              }
            "#;
            gl::ShaderSource(
                fs,
                1,
                &(FRAG_SHADER.as_bytes().as_ptr().cast()),
                &(FRAG_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(fs);
            let mut success = 0;
            gl::GetShaderiv(fs, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    fs,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vs);
            gl::AttachShader(shader_program, fs);
            gl::LinkProgram(shader_program);
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                    shader_program,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::UseProgram(shader_program);
            let pv_loc = gl::GetUniformLocation(shader_program, b"pv\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(
                pv_loc,
                1,
                gl::FALSE,
                &camera.pv_mat()[0][0]
            );
            let model_loc = gl::GetUniformLocation(shader_program, b"model\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(
                model_loc,
                1,
                gl::FALSE,
                &t_mat[0][0]
            );
            
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event, &mut player, &mut camera, &mut keystates);
            }
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
            glfw.poll_events();
            window.swap_buffers();
        }
        player.mv(glm::vec3( (keystates[1]-keystates[3])as f32*-MOVEMENT_DELTA, 0.0,(keystates[0]-keystates[2])as f32*-MOVEMENT_DELTA));
        camera.center[0] -= CAMERA_DELTA*(keystates[5]-keystates[7])as f32;
        camera.center[2] -= CAMERA_DELTA*(keystates[4]-keystates[6])as f32;
        camera.eye[2] -= CAMERA_DELTA*(keystates[4]-keystates[6])as f32;
        camera.mvhelper(player.pos,player.vec);
        player.mvhelper();
 

    }
}

fn handle_key_event(window: &mut glfw::Window, key: Key, action: Action, keystates: &mut [i32; 8]) {
    let index = match key {
        Key::W => 0, //modular mapping system
        Key::A => 1,
        Key::S => 2,
        Key::D => 3,
        Key::I => 4,
        Key::J => 5,
        Key::K => 6,
        Key::L => 7,
        Key::Escape => {
            window.set_should_close(true);
            return;
        }
        _ => {
            999
        }
    };
    if index != 999 {
        keystates[index] = if action == Action::Press { 1 } else { 0 };
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, player: &mut player::Player, camera:&mut camera::Camera,
    keystates:&mut [i32; 8]) {
    if let glfw::WindowEvent::Key(key, _, action, _) = event {
        handle_key_event(window, key, action, keystates);
    }
}
