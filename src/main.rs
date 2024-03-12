extern crate glfw;
extern crate gl;
extern crate glm;
extern crate num_traits;
use glfw::{Action, Context, Key};
pub mod camera; // camera stuff
pub mod meshloader; 
pub mod player; 

const ORIGIN: glm::Vector3<f32> = glm::Vector3{ x: 0.0, y: 0.0, z: 0.0 };
const MOVEMENT_DELTA: f32 = 0.05;

pub fn main() {
    let mut sphere = meshloader::Mesh{vertices: Vec::new()};
    sphere.load("assets/mesh/small_sphere.stl");
    let mut player = player::Player{mesh: sphere, pos: ORIGIN};
    camera::main();
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(800, 800, "Se-Phere!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        let mut vertices = Vec::new();
        for vertex in &player.t_mesh().vertices {
            //let proj = glm::ext::perspective::<f32>(1.0, 1.0, 0.0, 20.0);
            //let transformed = proj * vertex.extend(1.0);
            //vertices.extend_from_slice(&mut transformed.truncate(3).as_array().as_slice());
            vertices.extend_from_slice(&mut vertex.as_array().as_slice());
        }
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
              layout (location = 0) in vec3 pos;
              void main() {
                gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
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
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
            gl::UseProgram(shader_program);
            window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut player);
        }
        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
        }
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, player: &mut player::Player) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
    match event {
        glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
            player.mv(glm::vec3(0.0,MOVEMENT_DELTA,0.0));
        }
        _ => {}
    }
    match event {
        glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
            player.mv(glm::vec3(0.0,-MOVEMENT_DELTA,0.0));
        }
        _ => {}
    }
    match event {
        glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
            player.mv(glm::vec3(-MOVEMENT_DELTA,0.0,0.0));
        }
        _ => {}
    }
    match event {
        glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
            player.mv(glm::vec3(MOVEMENT_DELTA,0.0,0.0));
        }
        _ => {}
    }
}
