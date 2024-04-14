use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{env, io};
use tokio::net::{UdpSocket, TcpStream};
use glam::*;
use messaging::{Message, Command, AsBytes};

#[derive(Debug, Copy, Clone)]
struct Player {
    pid: u8,
    pos: Vec3A,
}

struct Ground {
    mutations: [f32; 65536],
    frame: u64,
}

struct GameState {
    players: [Option<Player>; 64],
    ground: Ground,
    num_players: u8,
}

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    state: GameState,
}

impl Server {
    async fn run(mut self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
            ref state,
        } = self;

        let mut player_sockets = vec!();
        let mut player_buffers: [Option<Vec<u8>>; 32] = Default::default();
        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let b = &buf[..size];
                if let Some(mut m) = Message::try_from_data(peer, b) {
                    //println!("Received command {:?} from {}", m.command, peer);
                    match m.command {
                        Command::LOGIN => {
                            let mut reply = Message::new(Command::SET_PID);
                            reply.push_bytes((player_sockets.len() as u8).as_bytes());
                            if let Some(port) = m.extract_u32(0) {
                                let ls = SocketAddr::new(peer.ip(), port as u16);
                                player_sockets.push(ls);
                                socket.send_to(&reply.get_bytes(), &peer).await?;
                                self.state.num_players += 1;
                            }
                            dbg!(&player_sockets);
                        },
                        Command::BLOB => {},
                        Command::STATE => {
                            if let Some(pid) = m.extract_u8(0) {
                                //println!("PID {} wants the gamestate", pid);
                                let mut reply = Message::new(Command::R_STATE);
                                reply.push_bytes(self.state.num_players.as_bytes());
                                reply.push_bytes(self.state.ground.frame.as_bytes());
                                socket.send_to(&reply.get_bytes(), &peer).await?;
                                /*
                                if let Some(v) = &player_buffers[pid as usize] {
                                    dbg!(v);
                                    socket.send_to(v.as_slice(), &peer).await?;
                                    player_buffers[pid as usize] = None; 
                                }*/
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        }
                        Command::POS => {
                            if let Some(pos) = m.extract_vec3a(1) {
                                let pid = m.extract_u8(0).unwrap();
                                if let Some(p) = &mut self.state.players[pid as usize] {
                                    *p.pos = *pos;
                                } else {
                                    let p = Some(Player {
                                        pid,
                                        pos,
                                    });
                                    self.state.players[pid as usize] = p;
                                }
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        }
                        Command::MUT => {
                            if let Some(amt) = m.extract_f32(5) {
                                let pid = m.extract_u8(0).unwrap();
                                let idx = m.extract_u32(1).unwrap();
                                self.state.ground.frame += 1;
                                //self.state.ground.mutations[idx as usize] += amt;
                                //println!("PID {} wants to mutate idx {} by {}", pid, idx, amt);

                                let data = m.get_bytes();
                                for p in 0..self.state.num_players {
                                    let mut v = vec!();
                                    v.extend_from_slice(&data);
                                    player_buffers[p as usize] = Some(v);
                                }
                                for ps in &player_sockets {
                                    socket.send_to(&data, &ps).await?;
                                }

                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }

                        },
                        Command::PPOS => {
                            if let Some(pid) = m.extract_u8(0) {
                                let mut reply = Message::new(Command::R_PPOS);
                                reply.push_bytes(self.state.num_players.as_bytes());
                                for i in 0..self.state.num_players {
                                    if let Some(p) = self.state.players[i as usize] {
                                        reply.push_bytes(p.pos.as_bytes());
                                    }
                                }
                                socket.send_to(&reply.get_bytes(), &peer).await?;
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        },
                        Command::GNDSTATE => {
                            if let Some(pid) = m.extract_u8(0) {
                                if let Some(v) = &player_buffers[pid as usize] {
                                    socket.send_to(v.as_slice(), &peer).await?;
                                    player_buffers[pid as usize] = None; 
                                } else {
                                    socket.send_to(&[], &peer).await?;
                                }
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        },
                        _ => { dbg!(&m); },
                    }
                } else {
                    println!("Invalid Command 0x{:02x}", b[0]);
                }
                //println!();
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:42069".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
        state: GameState {
            players: [None; 64],
            ground: Ground {
                mutations: [0f32; 65536],
                frame: 0u64,
            },
            num_players: 0u8,
        },
    };

    // This starts the server task.
    server.run().await?;
    Ok(())
}
