//#![allow(non_snake_case)]
use glam::{vec3a, vec4};
use glam::Vec3Swizzles;
use glam::f32:: Vec3A;
use glfw::Context;
use glfw::Cursor;
use glfw::StandardCursor::*;
pub mod shader;
use shader::ShaderProgram;
pub mod camera; 
pub mod meshloader; 
pub mod entities; 
use entities::*;
pub mod keys;
use std::{str, thread, time, f32::consts::PI};
use rand::*;

// net, tokio
use std::net::{Ipv4Addr,  SocketAddrV4};
use tokio::net::UdpSocket;

const DELTA_TIME: time::Duration = time::Duration::from_millis(16);

const ORIGIN: Vec3A = vec3a(0.0, 0.0, 0.0);
const MOVEMENT_DELTA: f32 = 0.006;
const CAMERA_DELTA: f32 = 0.03;
const PAN_TRESHOLD_RATIO:f64=0.1; //how close to the edge before panning
const TILT_TRESHOLD_RATIO:f64=0.1; //how close to the edge before tilting
const ZOOM_DELTA:f32 = 0.1;
const GROUND_IMMUTABLE_RADIUS: f32 = 0.5;
const PLAYER_SPAWN_RADIUS: f32 = 10.0;
const CUBE_SPAWN_RADIUS: f32 = 5.0;
const CUBE_RESPAWN_TIME: u64 = 60;
const MAX_LIGHTS: usize = 16;
const SERVER_IP_ADDR: Ipv4Addr = Ipv4Addr::new(127,0,0,1);
const SERVER_PORT: u16 = 42069;
const SERVER_SOCKET: SocketAddrV4 = SocketAddrV4::new(SERVER_IP_ADDR, SERVER_PORT);

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Create UDP socket
    let socket = UdpSocket::bind(SERVER_IP_ADDR.to_string()+":0").await?;
    socket.connect(SERVER_SOCKET).await?;

    let mut scr_w = 1920i32;
    let mut scr_h = 1080i32;
    let mut framenum = 0u64;
    let mut cube_respawn_frame = 0u64;
    
    let mut rng = thread_rng();

    let theta = rng.gen_range(0.0..2.0*PI);
    let player_init_pos = vec3a(PLAYER_SPAWN_RADIUS*f32::cos(theta), 0.1, PLAYER_SPAWN_RADIUS*f32::sin(theta));
    let player_init_cam = camera::PlayerCamera::new(player_init_pos, scr_w as f32/scr_h as f32, f32::atan2(player_init_pos.x, player_init_pos.z));
    // initializing entities as Entity
    let mut player = Player::new(
        "assets/mesh/sephere.stl",
        player_init_pos,
        1.0*vec3a(0.1, 0.5, 0.2),
        player_init_cam,
        1.0,
    );

    let mut cube = Entity::new(
        "assets/mesh/rt_marker.stl",
        ORIGIN + vec3a(0.0,0.0,0.0),
        1.0*vec3a(0.2, 0.2, 0.2),
        1.0,
    );
    cube.set_scale(2.0,2.0,2.0);

    let theta2 = rng.gen_range(0.0..2.0*PI);
    let cube_r = rng.gen_range(2.0..=CUBE_SPAWN_RADIUS);
    let cube_pos = vec3a(cube_r*f32::cos(theta2), -0.5, cube_r*f32::sin(theta2));

    let mut cube2 = Entity::new(
        "assets/mesh/mickey.stl",
        cube_pos,
        vec3a(0.8, 0.1, 0.8),
        1.0,
    );
    cube2.set_scale(0.7,0.7,0.7);
    cube2.reflectance = 0.4;

    let mut ground = Entity::new(
        "assets/mesh/ground.stl",
        ORIGIN,
        0.8*vec3a(0.47, 0.41, 0.34),
        0.0,
    );

    ground.set_scale(3.0,1.0,3.0);

    /* Add ground vertex marker cubes, uncomment to debug
    let mut ground_vertex_markers = vec!();
    for vertex in &ground.mesh.vertices {
        let mut marker = Entity::new(
            "assets/mesh/cube.stl",
            *vertex,
            vec3a(0.8, 0.2, 0.8),
            0.0,
            );
        marker.set_scale(0.2,0.2,0.2);
        ground_vertex_markers.push(marker);
    }
    */

    let mut rt_marker = Entity::new(
        "assets/mesh/rt_marker.stl",
        vec3a(0.0,0.3,0.0),
        vec3a(0.2, 0.2, 0.2),
        0.0,
    );
    rt_marker.set_scale(0.2,1.0,0.2);

    //keys
    let mut keystates = [0;16];
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.with_primary_monitor(|glfw, m| {
        glfw.create_window(1920, 1080, "Se-phere!",
            m.map_or(glfw::WindowMode::Windowed, |m| glfw::WindowMode::FullScreen(m)))
    }).expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor(Some(Cursor::standard(VResize)));
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();
    gl::load_with(|f_name| window.get_proc_address(f_name));

    let lighting_program: ShaderProgram;

    // lighting 
    let mut light_colors: [f32; MAX_LIGHTS*3] = [0.0; MAX_LIGHTS*3];
    let mut light_positions: [f32; MAX_LIGHTS*3] = [0.0; MAX_LIGHTS*3];

    let light_dist = 100f32;

    for i in 0..MAX_LIGHTS {
        let theta: f32 = 2.0*PI/(MAX_LIGHTS as f32) * (i as f32);
        let (x,y,z) = (light_dist*f32::cos(theta), 50.0, light_dist*f32::sin(theta));
        light_positions[3*i] = x;
        light_positions[3*i+1] = y;
        light_positions[3*i+2] = z;
        light_colors[3*i] = 4.0/(MAX_LIGHTS as f32);
        light_colors[3*i+1] = 4.0/(MAX_LIGHTS as f32);        
        light_colors[3*i+2] = 4.0/(MAX_LIGHTS as f32);        
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        lighting_program = ShaderProgram::new("src/shaders/lighting.vs", "src/shaders/lighting.fs");

        lighting_program.set_vec3fv(b"lightColor\0", MAX_LIGHTS, &light_colors[0]);
        lighting_program.set_vec3fv(b"lightPos\0", MAX_LIGHTS, &light_positions[0]);

        player.entity.gl_init();
        cube.gl_init();
        cube2.gl_init();
        ground.gl_init();
        rt_marker.gl_init();

        /* initialize ground vertex marker cubes
           for marker in &mut ground_vertex_markers {
           marker.gl_init();
           }
       */
    }


    while !window.should_close() {
        glfw.poll_events();
        window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut glfw, &mut window, event, &mut player, &mut keystates);
        }

        (scr_w, scr_h) = window.get_size();
        player.camera.aspect = scr_w as f32 / scr_h as f32;

        unsafe{ gl::Viewport(0,0,scr_w,scr_h) }

        // ground mesh selection / mouse tracking using rt_marker
        let (x, y) = window.get_cursor_pos();

        let mut i: usize = 0;

        let p = player.camera.proj_mat();
        let v = player.camera.view_mat();
        let pvi = (p*v).inverse();
        let ndc_x = (x as f32/scr_w as f32 - 0.5) * 2.0;
        let ndc_y = (y as f32/scr_h as f32 - 0.5) * -2.0;
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
        let eye = player.camera.eye();
        let raydir: Vec3A = (rew-rsw).normalize().into();

        if raydir.dot(vec3a(0.0,1.0,0.0)) < 0.0 {
            i = ground.closest_vertex_index(rt_marker.pos.xz());
            let ground_y = ground.mesh.vertices[i].y;
            rt_marker.pos = eye - raydir*(eye.y-ground_y)/raydir.y;
            if rt_marker.pos.xz().distance(ORIGIN.xz()) < GROUND_IMMUTABLE_RADIUS {
                window.set_cursor(Some(Cursor::standard(Arrow)));
            } else {
                window.set_cursor(Some(Cursor::standard(VResize)));
            }
        }

        if keystates[10] == 1 || keystates [11] == 1 {
            if rt_marker.pos.xz().distance(ORIGIN.xz()) >= GROUND_IMMUTABLE_RADIUS {
                if keystates[10] == 1 && keystates[11] == 0 {
                    ground.mesh.mutate(i, vec3a(0.0,1.0,0.0), player.ability.ground_mut_power);
                }
                if keystates[10] == 0 && keystates[11] == 1 {
                    ground.mesh.mutate(i, vec3a(0.0,-1.0,0.0), player.ability.ground_mut_power);
                }
            }
        }


        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            player.entity.draw(&mut player.camera, &lighting_program);

            ground.draw(&mut player.camera, &lighting_program);

            /* Debug raycasting using ground vertex markers 
            let mut j = 0 as usize;
            for marker in &mut ground_vertex_markers {
                if i==j {
                    if keystates[10] == 1 && keystates[11] == 0 {
                        marker.set_color(vec3a(0.8, 0.2, 0.0));
                    }
                    if keystates[10] == 0 && keystates[11] == 1 {
                        marker.set_color(vec3a(0.8, 0.2, 0.8));
                    }
                } else {
                    marker.set_color(vec3a(0.1,0.8,0.1));
                }
                marker.draw(&mut player.camera, &lighting_program);
                j += 1;
            }
            */
            
            cube.draw(&mut player.camera, &lighting_program);
            cube2.draw(&mut player.camera, &lighting_program);

            //rt_marker.draw(&mut player.camera, &lighting_program);
        }
        
        // player loop
        player.mv(vec3a(
                (keystates[0]-keystates[2]) as f32*-MOVEMENT_DELTA*f32::sin(player.camera.camera_angle), 0.0, // use camera angle as direction
                (keystates[0]-keystates[2]) as f32*-MOVEMENT_DELTA*f32::cos(player.camera.camera_angle))); // for the player to move towards
        player.mvhelper(); 

        if player.detect_col(&ground).0 {
            player.collide(&ground);
            player.on_ground = true;
        };
        if player.detect_col(&cube2).0 {
            player.collide(&cube2);
            cube_respawn_frame = framenum + CUBE_RESPAWN_TIME;
        };
        if framenum == cube_respawn_frame {
            let theta2 = rng.gen_range(0.0..2.0*PI);
            let cube_r = rng.gen_range(2.0..=CUBE_SPAWN_RADIUS);
            cube2.pos = vec3a(cube_r*f32::cos(theta2), -0.05, cube_r*f32::sin(theta2));
            let R = rng.gen_range(0.0..1.0);
            let G = rng.gen_range(0.0..1.0);
            let B = rng.gen_range(0.0..1.0);
            cube2.color = vec3a(R,G,B);
        }

        if player.detect_col(&cube).0 || player.entity.pos.y < -5.0 {
            let theta = rng.gen_range(0.0..2.0*PI);
            let player_init_pos = vec3a(PLAYER_SPAWN_RADIUS*f32::cos(theta), 0.5, PLAYER_SPAWN_RADIUS*f32::sin(theta));
            let player_init_cam = camera::PlayerCamera::update(player_init_pos, scr_w as f32/scr_h as f32, 
                f32::atan2(player_init_pos.x, player_init_pos.z),player.camera);
            player.entity.set_pos(player_init_pos);
            player.camera = player_init_cam;
            player.vec = vec3a(0.0,0.0,0.0);
        }

        // player.camera loop
        player.camera.player_pos=player.pos();
        player.camera.camera_angle += (if f32::abs(player.vec.x) > 0.0001 || f32::abs(player.vec.z) > 0.0001 {1}
            // allows spin only if player vec is > 0
            else {0}) as f32 * CAMERA_DELTA * (keystates[1]-keystates[3]) as f32; // ks[1]-ks[3] as a & d keys - left/right
        player.camera.tilt+= CAMERA_DELTA*(keystates[4]-keystates[5]) as f32; // ks[4]-ks[5] as i & k keys - pan up/pan down


        // //mouse control
        if x < scr_w as f64 * PAN_TRESHOLD_RATIO{
            player.camera.camera_angle += CAMERA_DELTA;
        }
        else if x > scr_w as f64 * (1.0-PAN_TRESHOLD_RATIO){
            player.camera.camera_angle -= CAMERA_DELTA;
        }

        if y < scr_h as f64 * TILT_TRESHOLD_RATIO{
            player.camera.tilt -= CAMERA_DELTA;
        }
        else if y > scr_h as f64 * (1.0-TILT_TRESHOLD_RATIO){
            player.camera.tilt += CAMERA_DELTA;
        }


        // socket
        

        let s = format!("POS: {:?}", &player.pos()); 
        socket.send(s.as_bytes()).await?;

        let mut buf = vec![0; 1024];
        let (size, _peer) = socket.recv_from(&mut buf).await?;
        let s = str::from_utf8(&buf[..size]).unwrap();
        println!("Reply from server: {}",s);

        window.swap_buffers();
        thread::sleep(DELTA_TIME);
        framenum += 1;
        //dbg!(player.on_ground);
    }

    Ok(())

}


fn handle_window_event(glfw: &mut glfw::Glfw, window: &mut glfw::Window, event: glfw::WindowEvent,player: &mut Player, keystates:&mut [i8; 16]) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::F, _, glfw::Action::Press, _) => {
            let mut fullscreen = false;
            window.with_window_mode(|mode| {
                match mode {
                    glfw::WindowMode::Windowed => fullscreen = false,
                    glfw::WindowMode::FullScreen(_) => fullscreen = true,
                }
            });
            if fullscreen {
                window.set_monitor(glfw::WindowMode::Windowed,0,0,800,600,Some(60));
            } else {
                glfw.with_primary_monitor(|_,m| {
                    window.set_monitor(glfw::WindowMode::FullScreen(m.expect("Failed to set Fullscreen")),0,0,1920,1080,Some(60));
                });
            }
        }
        // jump
        glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
            if player.on_ground {
                player.on_ground = false;
                player.vec.y += 0.1;
            }
        }
        glfw::WindowEvent::Key(key,_,action,modifier) =>{
            keys::handle_key_event(window, key, action,modifier, keystates);}

        glfw::WindowEvent::MouseButton(mouse_button,action,modifier) =>{
            keys::handle_mouse_button(mouse_button,action,modifier,keystates);}

        glfw::WindowEvent::Scroll(_x,y)=>{
                player.camera.radius -= ZOOM_DELTA*y as f32;}

        _=>{}
    } 

}