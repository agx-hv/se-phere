pub mod camera;
pub mod entities;
pub mod keys;
pub mod meshloader;
pub mod music;
pub mod shader;

use entities::*;
use meshloader::Mesh;
use rand::{thread_rng, Rng};
use shader::ShaderProgram;

use glam::f32::Vec3A;
use glam::Vec3Swizzles;
use glam::{vec3a, vec4};
use glfw::Context;
use glfw::Cursor;
use glfw::StandardCursor::*;
use rand::rngs::ThreadRng;
use std::{env, f32::consts::PI, time};

// sound
use rodio::OutputStream; // dont remove import, for comme

// net, tokio, messaging
use messaging::{AsBytes, Command, Message};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use tokio::net::UdpSocket;

const DELTA_TIME: time::Duration = time::Duration::from_millis(1);
const ORIGIN: Vec3A = vec3a(0.0, 0.0, 0.0);
const MOVEMENT_DELTA: f32 = 0.005;
const CAMERA_DELTA: f32 = 0.03;
const PAN_TRESHOLD_RATIO: f64 = 0.01; //how close to the edge before panning
const TILT_TRESHOLD_RATIO: f64 = 0.01; //how close to the edge before tilting
const ZOOM_DELTA: f32 = 0.1;
const GROUND_IMMUTABLE_RADIUS: f32 = 1.5;
const PLAYER_SPAWN_RADIUS: f32 = 10.0;
// const CUBE_SPAWN_RADIUS: f32 = 5.0;
// const CUBE_RESPAWN_TIME: u64 = 60;
const MAX_LIGHTS: usize = 16;
const LOCAL_IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
const SERVER_PORT: u16 = 42069;
const ENEMY_COLOR: Vec3A = vec3a(0.6, 0.1, 0.8);

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // initialize varibles for starting
    let counter = Arc::new(AtomicU64::new(0));
    let num_players = Arc::new(AtomicU8::new(0));
    let mut player_positions = vec![];
    for _ in 0..64 {
        player_positions.push((
            Arc::new(AtomicU8::new(255)),
            Arc::new(AtomicU32::new(0)),
            Arc::new(AtomicU32::new(0)),
            Arc::new(AtomicU32::new(0)),
        ));
    }
    let mut gnd_muts = vec![];
    gnd_muts.push((Arc::new(AtomicU32::new(0)), Arc::new(AtomicU32::new(64))));
    for _ in 1..64 {
        gnd_muts.push((Arc::new(AtomicU32::new(0)), Arc::new(AtomicU32::new(0))));
    }

    let mut args = vec![];
    for arg in env::args() {
        args.push(arg);
    }

    let server_socket: SocketAddr = SocketAddr::new(
        args[1]
            .parse()
            .expect(&format!("Invalid IP: {}", args[1]).to_string()),
        SERVER_PORT,
    );

    // Create UDP socket
    let socket = UdpSocket::bind(LOCAL_IP_ADDR.to_string() + ":0").await?;
    socket.connect(server_socket).await?;

    let listener = UdpSocket::bind(LOCAL_IP_ADDR.to_string() + ":0").await?;
    listener.connect(server_socket).await?;

    let SocketAddr::V4(v4) = &listener.local_addr().unwrap() else {
        unreachable!()
    };

    let mut m = Message::new(Command::LOGIN);
    m.push_bytes((v4.port() as u32).as_bytes());
    socket.send(&m.get_bytes()).await?;

    let mut buf = vec![0; 1024];
    let (size, peer) = socket.recv_from(&mut buf).await?;
    let m = Message::try_from_data(peer, &buf[..size]).unwrap();
    match m.command {
        Command::SETPID => {
            let pid = m.extract_u8(0).unwrap();
            let _ = tokio::join!(
                game(
                    &socket,
                    counter.clone(),
                    &listener,
                    pid,
                    num_players.clone(),
                    gnd_muts.clone(),
                    player_positions.clone()
                ),
                listen(
                    &socket,
                    counter.clone(),
                    num_players.clone(),
                    pid,
                    gnd_muts.clone(),
                    player_positions.clone()
                ),
            );
        }
        _ => todo!(),
    }

    Ok(())
}

async fn listen(
    socket: &UdpSocket,
    counter: Arc<AtomicU64>,
    num_players: Arc<AtomicU8>,
    pid: u8,
    gnd_muts: Vec<(Arc<AtomicU32>, Arc<AtomicU32>)>,
    player_positions: Vec<(
        Arc<AtomicU8>,
        Arc<AtomicU32>,
        Arc<AtomicU32>,
        Arc<AtomicU32>,
    )>,
) -> tokio::io::Result<()> {
    loop {
        let mut m = Message::new(Command::STATE);
        m.push_bytes(pid.as_bytes());
        socket.send(&m.get_bytes()).await?;

        let mut m = Message::new(Command::PPOS);
        m.push_bytes(pid.as_bytes());
        socket.send(&m.get_bytes()).await?;

        let mut m = Message::new(Command::GNDSTATE);
        m.push_bytes(pid.as_bytes());
        socket.send(&m.get_bytes()).await?;

        let mut buf = vec![0; 1024];
        let (size, peer) = socket.recv_from(&mut buf).await?;
        let m = Message::try_from_data(peer, &buf[..size]).unwrap();
        match m.command {
            Command::RSTATE => {
                num_players.store(m.extract_u8(0).unwrap(), Ordering::Relaxed);
                let num_mutations = m.extract_u64(1).unwrap();
                if counter.load(Ordering::Relaxed) < num_mutations {
                    counter.store(num_mutations, Ordering::Relaxed);
                    //println!("{:?}", counter);
                }
            }
            Command::RPPOS => {
                if let Some(np) = m.extract_u8(0) {
                    for idx in 0..np {
                        if let Some(ppos) = m.extract_vec3a((12 * idx + 1).into()) {
                            let (i, x, y, z) = &player_positions[idx as usize];
                            i.store(idx.into(), Ordering::Relaxed);
                            x.store(ppos.x.to_bits().into(), Ordering::Relaxed);
                            y.store(ppos.y.to_bits().into(), Ordering::Relaxed);
                            z.store(ppos.z.to_bits().into(), Ordering::Relaxed);
                        }
                    }
                }
            }
            Command::MUT => {
                if let Some(amt) = m.extract_f32(5) {
                    let idx = m.extract_u32(1).unwrap();
                    let (i, a) = &gnd_muts[1];
                    i.store(idx.into(), Ordering::Relaxed);
                    a.store(amt.to_bits().into(), Ordering::Relaxed);
                    //dbg!(&gnd_muts[1]);
                }
            }
            _ => todo!(),
        }
    }
}

async fn game(
    socket: &UdpSocket,
    counter: Arc<AtomicU64>,
    listener: &UdpSocket,
    pid: u8,
    num_players: Arc<AtomicU8>,
    gnd_muts: Vec<(Arc<AtomicU32>, Arc<AtomicU32>)>,
    player_positions: Vec<(
        Arc<AtomicU8>,
        Arc<AtomicU32>,
        Arc<AtomicU32>,
        Arc<AtomicU32>,
    )>,
) -> tokio::io::Result<()> {
    let mut scr_w = 1920i32;
    let mut scr_h = 1080i32;
    let mut framenum = 0u64;

    let mut rng = thread_rng();

    let theta = rng.gen_range(0.0..2.0 * PI);
    let player_init_pos = vec3a(
        PLAYER_SPAWN_RADIUS * f32::cos(theta),
        0.1,
        PLAYER_SPAWN_RADIUS * f32::sin(theta),
    );
    let player_init_cam = camera::PlayerCamera::new(
        player_init_pos,
        scr_w as f32 / scr_h as f32,
        f32::atan2(player_init_pos.x, player_init_pos.z),
    );

    let mut myscore = 0;
    let mut myhealth = 3;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // initializing entities as Entity
    let mut player = Player::new(
        "assets/mesh/small_sphere.stl",
        player_init_pos,
        1.0 * vec3a(0.1, 0.5, 0.2),
        player_init_cam,
        1.0,
        pid,
    );

    let mut goal_2d = Entity::new("assets/mesh/3.stl", ORIGIN, 1.0 * vec3a(0.2, 0.2, 0.2), 1.0);
    goal_2d.set_scale(0.01, 0.01, 0.01);

    let mut goal = Entity::new(
        "assets/mesh/rt_marker.stl",
        ORIGIN + vec3a(0.0, 0.0, 0.0),
        1.0 * vec3a(0.8, 0.8, 0.2),
        1.0,
    );
    goal.set_scale(2.0, 2.0, 2.0);

    // let theta2 = rng.gen_range(0.0..2.0*PI);
    // let cube_r = rng.gen_range(2.0..=CUBE_SPAWN_RADIUS);
    // let cube_pos = vec3a(cube_r*f32::cos(theta2), -0.5, cube_r*f32::sin(theta2));

    let mut other_player_entities = vec![];
    for _ in 0..pid {
        let newplayer = Entity::new(
            "assets/mesh/small_sphere.stl",
            ORIGIN,
            randcolor(&mut rng),
            1.0,
        );
        let score = Entity::new("assets/mesh/3.stl", ORIGIN, ENEMY_COLOR, 1.0);
        other_player_entities.push((newplayer, score));
    }

    let mut newplayer = Entity::new(
        "assets/mesh/small_sphere.stl",
        ORIGIN,
        vec3a(0.1, 0.1, 0.8),
        1.0,
    );
    newplayer.set_scale(0.0, 0.0, 0.0);
    let mut score = Entity::new("assets/mesh/3.stl", ORIGIN, vec3a(0.8, 0.1, 0.8), 1.0);
    score.set_scale(0.0, 0.0, 0.0);
    other_player_entities.push((newplayer, score));

    let mut score_stl = Entity::new("assets/mesh/0.stl", ORIGIN, vec3a(0.1, 0.5, 0.2), 1.0);

    let mut myhearts = vec![];
    for _ in 0..3 {
        let mut heart_stl =
            Entity::new("assets/mesh/heart.stl", ORIGIN, vec3a(0.8, 0.2, 0.2), 1.0); //spawn new heart with red colour
        heart_stl.set_scale(0.5, 0.5, 0.5);
        myhearts.push(heart_stl);
    }

    let mut ground = Entity::new(
        "assets/mesh/ground.stl",
        ORIGIN,
        vec3a(0.47, 0.41, 0.34),
        0.0,
    );
    ground.set_texture_id(1);
    ground.set_scale(3.0, 1.0, 3.0);
    ground.reflectance = 1.1;

    let mut rt_marker = Entity::new(
        "assets/mesh/rt_marker.stl",
        ORIGIN,
        1.0 * vec3a(0.2, 0.2, 0.2),
        1.0,
    );
    rt_marker.set_scale(2.0, 2.0, 2.0);

    //println!("{:?}", ground.mesh.vertices.len());

    // 1D noise test
    /*
    let mut noise_1d = [0.0;160];
    for i in 0..160 {
        noise_1d[i] = rng.gen_range(-0.05..0.05);
    }

    for dr in 30..160 {
        let r = 0.1*(dr as f32);
        for dtheta in 0..200 {
            let theta = 2.0*PI/200.0*(dtheta as f32);
            let xz = vec3a(r*f32::cos(theta), 0.0, r*f32::sin(theta)).xz();
            let idx = ground.closest_vertex_index(xz);
            ground.mesh.mutate(idx ,vec3a(0.0,-1.0,0.0),noise_1d[dr]);
        }
    }

    */
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

    //keys
    let mut keystates = [0; 16];
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            glfw.create_window(
                1920,
                1080,
                "Se-phere!",
                m.map_or(glfw::WindowMode::Windowed, |m| {
                    glfw::WindowMode::FullScreen(m)
                }),
            )
        })
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor(Some(Cursor::standard(VResize)));
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.make_current();
    gl::load_with(|f_name| window.get_proc_address(f_name));

    let lighting_program: ShaderProgram;

    // lighting
    let mut light_colors: [f32; MAX_LIGHTS * 3] = [0.0; MAX_LIGHTS * 3];
    let mut light_positions: [f32; MAX_LIGHTS * 3] = [0.0; MAX_LIGHTS * 3];

    let light_dist = 100f32;

    for i in 0..MAX_LIGHTS {
        let theta: f32 = 2.0 * PI / (MAX_LIGHTS as f32) * (i as f32);
        let (x, y, z) = (
            light_dist * f32::cos(theta),
            50.0,
            light_dist * f32::sin(theta),
        );
        light_positions[3 * i] = x;
        light_positions[3 * i + 1] = y;
        light_positions[3 * i + 2] = z;
        light_colors[3 * i] = 4.0 / (MAX_LIGHTS as f32);
        light_colors[3 * i + 1] = 4.0 / (MAX_LIGHTS as f32);
        light_colors[3 * i + 2] = 4.0 / (MAX_LIGHTS as f32);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.52, 0.81, 0.92, 1.0);
        lighting_program = ShaderProgram::new(
            "client/src/shaders/lighting.vs",
            "client/src/shaders/lighting.fs",
        );
        lighting_program.set_vec3fv(b"lightColor\0", MAX_LIGHTS, &light_colors[0]);
        lighting_program.set_vec3fv(b"lightPos\0", MAX_LIGHTS, &light_positions[0]);

        //init graphics (vertices, normals, textures)
        player.entity.gl_init();
        for (pe, score) in &mut other_player_entities {
            pe.gl_init();
            score.gl_init();
        }
        goal.gl_init();
        ground.gl_init();
        rt_marker.gl_init();
        goal_2d.gl_init();
        score_stl.gl_init();

        for e in &mut myhearts {
            e.gl_init();
        }

        /* initialize ground vertex marker cubes
            for marker in &mut ground_vertex_markers {
            marker.gl_init();
            }
        */
    }

    //loop
    while !window.should_close() {
        //increase frame number
        framenum += 1;
        
        glfw.poll_events();
        window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut glfw, &mut window, event, &mut player, &mut keystates);
        }

        //update aspect ratio to window size
        (scr_w, scr_h) = window.get_size();
        player.camera.camera_bare.aspect = scr_w as f32 / scr_h as f32;
        unsafe { gl::Viewport(0, 0, scr_w, scr_h) }

        let (x, y) = window.get_cursor_pos();
        
        // ground mesh selection / mouse tracking using rt_marker
        let raycast = cursor_raycast_and_animate(&mut player, x, scr_w, y, scr_h, &mut ground, &mut rt_marker, &mut window);

        //send intention to mutate
        if keystates[10] == 1 || keystates[11] == 1 {
            if rt_marker.pos.xz().distance(ORIGIN.xz()) >= GROUND_IMMUTABLE_RADIUS {
                let mut m = Message::new(Command::MUT);
                if keystates[10] == 1 && keystates[11] == 0 {
                    m.push_bytes(player.player_id.as_bytes());
                    m.push_bytes((raycast as u32).as_bytes());
                    m.push_bytes(player.ability.ground_mut_power.as_bytes());
                    listener.send(&m.get_bytes()).await?;
                }
                if keystates[10] == 0 && keystates[11] == 1 {
                    m.push_bytes(player.player_id.as_bytes());
                    m.push_bytes((raycast as u32).as_bytes());
                    m.push_bytes((-player.ability.ground_mut_power).as_bytes());
                    listener.send(&m.get_bytes()).await?;
                }
            }
        }

        //mutate mesh
        let gvec = &gnd_muts;
        let (idx, amt) = &gvec[1];
        ground.mesh.mutate(
            idx.load(Ordering::Relaxed) as usize,
            vec3a(0.0, 1.0, 0.0),
            f32::from_bits(amt.load(Ordering::Relaxed)),
        );
        idx.store(0, Ordering::Relaxed);
        amt.store(0, Ordering::Relaxed);
        
        // player movement
        player.mv(vec3a(
            (keystates[0] - keystates[2]) as f32
                * -MOVEMENT_DELTA
                * f32::sin(player.camera.camera_angle),
            0.0, // use camera angle as direction
            (keystates[0] - keystates[2]) as f32
                * -MOVEMENT_DELTA
                * f32::cos(player.camera.camera_angle),
        )); // for the player to move towards
        player.mvhelper();
        
        //init emeny spheres
        let pvec = &player_positions;
        let mut np = 0; // number of players
        for p in pvec {
            if p.0.load(Ordering::Relaxed) == 255 {
                break;
            }
            np += 1;
        }
        if np > other_player_entities.len() as u8 {
            let mut newplayer = Entity::new(
                "assets/mesh/small_sphere.stl",
                ORIGIN,
                randcolor(&mut rng),
                1.0,
            );
            let mut score = Entity::new("assets/mesh/3.stl", ORIGIN, ENEMY_COLOR, 1.0);
            unsafe {
                newplayer.gl_init();
                score.gl_init();
            }
            other_player_entities.push((newplayer, score));
        }

        //get enemy position
        for p in pvec {
            let pid = p.0.load(Ordering::Relaxed);
            if pid == 255 {
                break;
            }
            if pid == player.player_id {
                continue;
            }
            let (_, ux, uy, uz) = p;
            let x = f32::from_bits(ux.load(Ordering::Relaxed));
            let y = f32::from_bits(uy.load(Ordering::Relaxed));
            let z = f32::from_bits(uz.load(Ordering::Relaxed));
            other_player_entities[pid as usize].0.pos = vec3a(x, y, z);
            other_player_entities[pid as usize].1.pos = vec3a(x, y, z) + vec3a(0.0, 0.3, 0.0);
        }
        
        /* 
        
        COLLISION DETECTION (check under camera control for collide camera, it has to be there)

         */
        

        //collision detection with goal
        let has_goal = player.detect_col(&goal).0;

        //collision detection for ground
        if player.detect_col(&ground).0 {
            player.collide(&ground);
            player.on_ground = true;
        };

        //collision detection for other players
        for i in 0..other_player_entities.len() {
            if i != player.player_id.into() {
                if player.detect_col(&other_player_entities[i].0).0 {
                    player.collide(&other_player_entities[i].0);
                }
            }
            other_player_entities[i]
                .1
                .mesh
                .rotate_y(0.15 * framenum as f32);
        }
    
        /* 
        
        END COLLISION DETECTION

         */

        
        //respawn
        if has_goal || player.entity.pos.y < -5.0 {
            let theta = rng.gen_range(0.0..2.0 * PI);
            let theta2 = rng.gen_range(0.0..2.0 * PI);
            let player_init_pos = vec3a(
                PLAYER_SPAWN_RADIUS * f32::cos(theta),
                0.5,
                PLAYER_SPAWN_RADIUS * f32::sin(theta),
            );

            player.entity.set_pos(player_init_pos);
            let old_camera = player.camera;
            player.camera = camera::PlayerCamera::update(
                player_init_pos,
                scr_w as f32 / scr_h as f32,
                theta2,
                old_camera,
            );
            player.vec = vec3a(0.0, 0.0, 0.0);
            if has_goal {
                if myscore == 9 {
                    let _ = std::process::Command::new("target/release/image-ui")
                        .args(["win"])
                        .spawn();
                    break;
                }
                myscore += 1;
                let path = ["assets/mesh/", &myscore.to_string(), ".stl"].join("");
                score_stl.mesh = Mesh::new(&path, vec3a(1.0, 1.0, 1.0));
                music::play("assets/sounds/yay.mp3",&stream_handle);
            } else {
                myhealth -= 1;
                if myhealth == 0 {
                    let _ = std::process::Command::new("target/release/image-ui")
                        .args(["lose"])
                        .spawn();
                    break;
                }
                myhearts.pop();
                let path = ["assets/mesh/", &myscore.to_string(), ".stl"].join("");
                score_stl.mesh = Mesh::new(&path, vec3a(1.0, 1.0, 1.0));
                music::play("assets/sounds/oof.mp3",&stream_handle);
            }
        }

        // send position to server
        let p: [u8; 14] = player.pos_cmd();
        socket.send(&p).await?;

       //move camera to player
        player.camera.player_pos = player.pos();
        player.camera.camera_angle +=
            (if f32::abs(player.vec.x) > 0.0001 || f32::abs(player.vec.z) > 0.0001 {
                1
            }
            // allows spin only if player vec is > 0
            else {
                0
            }) as f32
                * CAMERA_DELTA
                * (keystates[1] - keystates[3]) as f32; // ks[1]-ks[3] as a & d keys - left/right
        
        //camera control
        if x < scr_w as f64 * PAN_TRESHOLD_RATIO {
            player.camera.camera_angle += CAMERA_DELTA;
        } else if x > scr_w as f64 * (1.0 - PAN_TRESHOLD_RATIO) {
            player.camera.camera_angle -= CAMERA_DELTA;
        }

        if y < scr_h as f64 * TILT_TRESHOLD_RATIO {
            let (c, _) = player.camera.detect_col(&ground);
            if !c && player.camera.eye().y > 0.2 {
                player.camera.tilt -= CAMERA_DELTA;
            }
        } else if y > scr_h as f64 * (1.0 - TILT_TRESHOLD_RATIO) {
            player.camera.tilt += CAMERA_DELTA;
        }        

        //collision detection for camera
        player.camera.collide(&ground);

        //score rotation
        score_stl.mesh.rotate_y(0.03 * framenum as f32);
        score_stl.pos = player.pos() + vec3a(0.0, 0.3, 0.0);
    

        //heart rotation
        for heart in &mut myhearts {
            heart.mesh.rotate_y(player.camera.camera_angle);
        }

        let offset = 0.13
            * player
            .camera
            .up()
            .cross(player.pos() - player.camera.eye())
            .normalize();

        for i in 0..myhearts.len() {
            myhearts[i].pos = player.pos() + vec3a(0.0, 0.5, 0.0) - offset * (i as f32 - 1.0);
        }


        //draw players
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            player
                .entity
                .draw(&mut player.camera, &lighting_program);

            ground.draw(&mut player.camera, &lighting_program);

            goal.draw(&mut player.camera, &lighting_program);
            score_stl.draw(&mut player.camera, &lighting_program);
            for e in &mut myhearts {
                e.draw(&mut player.camera, &lighting_program);
            }
            for (pe, _score) in &mut other_player_entities {
                pe.draw(&mut player.camera, &lighting_program);
            }
        }

        window.swap_buffers();
        tokio::time::sleep(DELTA_TIME).await;
    }


    Ok(())
}

fn cursor_raycast_and_animate(player: &mut Player, x: f64, scr_w: i32, y: f64, scr_h: i32, ground: &mut Entity, rt_marker: &mut Entity, window: &mut glfw::PWindow) -> usize {
    let mut raycast: usize = 0;
    let p = player.camera.proj_mat();
    let v = player.camera.view_mat();
    let pvi = (p * v).inverse();
    let ndc_x = (x as f32 / scr_w as f32 - 0.5) * 2.0;
    let ndc_y = (y as f32 / scr_h as f32 - 0.5) * -2.0;
    let rs = vec4(ndc_x, ndc_y, -1.0, 1.0);
    let re = vec4(ndc_x, ndc_y, 0.0, 1.0);
    let mut rsw = pvi * rs;
    rsw /= rsw[3];
    let mut rew = pvi * re;
    rew /= rew[3];
    let eye = player.camera.eye();
    let raydir: Vec3A = (rew - rsw).normalize().into();

    //set cursor animation
    if raydir.dot(vec3a(0.0, 1.0, 0.0)) < 0.0 {
        raycast = ground.closest_vertex_index(rt_marker.pos.xz());
        let ground_y = ground.mesh.vertices[raycast].y;
        rt_marker.pos = eye - raydir * (eye.y - ground_y) / raydir.y;
        if rt_marker.pos.xz().distance(ORIGIN.xz()) < GROUND_IMMUTABLE_RADIUS {
            window.set_cursor(Some(Cursor::standard(Arrow)));
        } else {
            window.set_cursor(Some(Cursor::standard(VResize)));
        }
    }
    raycast
}

fn handle_window_event(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    player: &mut Player,
    keystates: &mut [i8; 16],
) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::F, _, glfw::Action::Press, _) => {
            let mut fullscreen = false;
            window.with_window_mode(|mode| match mode {
                glfw::WindowMode::Windowed => fullscreen = false,
                glfw::WindowMode::FullScreen(_) => fullscreen = true,
            });
            if fullscreen {
                window.set_monitor(glfw::WindowMode::Windowed, 0, 0, 800, 600, Some(60));
            } else {
                glfw.with_primary_monitor(|_, m| {
                    window.set_monitor(
                        glfw::WindowMode::FullScreen(m.expect("Failed to set Fullscreen")),
                        0,
                        0,
                        1920,
                        1080,
                        Some(60),
                    );
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
        glfw::WindowEvent::Key(key, _, action, modifier) => {
            keys::handle_key_event(window, key, action, modifier, keystates);
        }

        glfw::WindowEvent::MouseButton(mouse_button, action, modifier) => {
            keys::handle_mouse_button(mouse_button, action, modifier, keystates);
        }

        glfw::WindowEvent::Scroll(_x, y) => {
            player.camera.radius -= ZOOM_DELTA * y as f32;
        }

        _ => {}
    }
}

fn randcolor(rng: &mut ThreadRng) -> Vec3A {
    let r = rng.gen_range(0.0..1.0);
    let g = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);
    return vec3a(r, g, b);
}
