extern crate glfw;
extern crate gl;
extern crate glam;
extern crate num_traits;
use glam::{dvec2, vec3a, vec4};
use glam::f32::{Vec2, Vec3A, Vec4};
use glam::f64::{DVec2};
use glfw::Context;
use std::f32::consts::PI;
pub mod shader;
use shader::ShaderProgram;
pub mod camera; 
pub mod meshloader; 
pub mod entities; 
use entities::*;
pub mod keys;
use std::{thread, time};

const DELTA_TIME: time::Duration = time::Duration::from_millis(16);

const ORIGIN: Vec3A = vec3a(0.0, 0.0, 0.0);
const MOVEMENT_DELTA: f32 = 0.001;
const CAMERA_DELTA: f32 = 0.01;
const SCR_W: f32 = 1920.0;
const SCR_H: f32 = 1080.0;
const PAN_TRESHOLD_RATIO:f64=0.1; //how close to the edge before panning
const TILT_TRESHOLD_RATIO:f64=0.1; //how close to the edge before tilting

pub fn main() {
    
    let mut ground_vertex_markers = vec!();

    let mut player = Player::new(
        "assets/mesh/small_sphere.stl",
        vec3a(0.1, 0.1, 0.3),
        vec3a(0.1, 0.5, 0.2));
    let mut cube = Entity::new(
        "assets/mesh/cube.stl",
        ORIGIN,
        vec3a(0.2, 0.1, 0.8));
    let mut ground = Entity::new(
        "assets/mesh/ground_lowpoly.stl",
        ORIGIN,
        vec3a(0.4, 0.2, 0.1));

    for vertex in &ground.mesh.vertices {
        if vertex[1] == 0.0 {
            let mut marker = Entity::new(
                "assets/mesh/cube.stl",
                *vertex,
                vec3a(0.8, 0.2, 0.8));
            ground_vertex_markers.push(marker);
        }
    }

    let mut rt_marker = Entity::new(
        "assets/mesh/cube.stl",
        vec3a(0.0,0.3,0.0),
        vec3a(0.1, 0.1, 0.1));

    let mut player_camera = camera::PlayerCamera {
        player_pos: vec3a(0.0, 1.0, 3.0),
        camera_angle: 0.0, // 0 to 2pi
        tilt: 0.6, //0 to pi
        radius: 2.0,
        fov: PI/3.0,
        aspect: SCR_W/SCR_H,
        near: 0.1,
        far: 100.0};

    //keys
    let mut keystates = [0;16];
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(SCR_W as u32, SCR_H as u32, "Se-Phere!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();
    gl::load_with(|f_name| window.get_proc_address(f_name));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        let lighting_program = ShaderProgram::new("src/shaders/lighting.vs", "src/shaders/lighting.fs");

        lighting_program.setVec3f(b"lightColor\0", 2.0, 2.0, 2.0);
        lighting_program.setVec3f(b"lightPos\0", 10.0, 25.0, 10.0);

        player.entity.gl_init();
        cube.gl_init();
        ground.gl_init();
        rt_marker.gl_init();
        for mut marker in &mut ground_vertex_markers {
            marker.gl_init();
        }

        while !window.should_close() {
            // ground mesh selection
            //mouse tracking
            let (x, y) = window.get_cursor_pos();

            if keystates[10] == 0 {
                let p = player_camera.proj_mat();
                let v = player_camera.view_mat();
                let pvi = (p*v).inverse();
                let ndc_x = (x as f32/SCR_W  - 0.5) * 2.0;
                let ndc_y = (y as f32/SCR_H - 0.5) * -2.0;
                let rs = vec4(
                    ndc_x,
                    ndc_y,
                    -1.0, 
                    1.0);
                let re = vec4(
                    ndc_x,
                    ndc_y,
                    0.0, 
                    1.0);
                let mut rsw = pvi*rs;
                rsw /= rsw[3];
                let mut rew = pvi*re;
                rew /= rew[3];
                let raydir: Vec3A = (rew-rsw).normalize().into();
                let eye = player_camera.eye();

                let delta = 0.0001f64;
                let mut i = 0f32;
                let mut p = eye;

                while num_traits::abs(p[1]) > 0.01 {
                    p += (delta as f32)*i*raydir;
                    i += 1.0;
                }
                rt_marker.pos = p;
                
            }

            let (width,height) = window.get_size(); //get window width and height
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            player.entity.draw(&mut player_camera, &lighting_program);
            //cube.draw(&mut player_camera, &lighting_program);
            ground.draw(&mut player_camera, &lighting_program);

            for marker in &mut ground_vertex_markers {
                marker.draw(&mut player_camera, &lighting_program);
            }

            rt_marker.draw(&mut player_camera, &lighting_program);

            glfw.poll_events();
            window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event, &mut keystates);
            }
            window.swap_buffers();
            
            // player loop
            player.mv(vec3a(
                (keystates[0]-keystates[2]) as f32*-MOVEMENT_DELTA*f32::sin(player_camera.camera_angle), 0.0, // use camera angle as direction
                (keystates[0]-keystates[2]) as f32*-MOVEMENT_DELTA*f32::cos(player_camera.camera_angle))); // for the player to move towards
            player.mvhelper(); 
            
            // player_camera loop
            player_camera.player_pos=player.pos();
            player_camera.camera_angle += (if f32::abs(player.vec.x) > 0.0001 || f32::abs(player.vec.z) > 0.0001 {1}
                else {keystates[0]-keystates[2]}) as f32 * CAMERA_DELTA * (keystates[1]-keystates[3]) as f32;
            player_camera.tilt+= CAMERA_DELTA*(keystates[4]-keystates[5]) as f32;


            /*
            //mouse control

            if x < width as f64 * PAN_TRESHOLD_RATIO{
                player_camera.camera_angle += CAMERA_DELTA;
            }
            else if x > width as f64 * (1.0-PAN_TRESHOLD_RATIO){
                player_camera.camera_angle -= CAMERA_DELTA;
            }

            if y < height as f64 * TILT_TRESHOLD_RATIO{
                player_camera.tilt -= CAMERA_DELTA;
            }
            else if y > height as f64 * (1.0-TILT_TRESHOLD_RATIO){
                player_camera.tilt += CAMERA_DELTA;
            }
            */


            thread::sleep(DELTA_TIME);
            //dbg!(player.pos);
        }


    }
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent,
    keystates:&mut [i8; 16]) {
    match event {
        glfw::WindowEvent::Key(key,_,action,modifier) =>{
            keys::handle_key_event(window, key, action,modifier, keystates);}

        glfw::WindowEvent::MouseButton(mouse_button,action,modifier) =>{
            keys::handle_mouse_button(mouse_button,action,modifier,keystates);}
        _=>{}
    } 

}
