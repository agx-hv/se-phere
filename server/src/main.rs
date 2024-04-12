use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{env, io};
use tokio::net::{UdpSocket, TcpStream};
use glam::*;
use messaging::{Message, Command, AsBytes};

struct Player {
    pos: Vec3A,
}

struct Ground {
    num_mutations: u64,
    mutations: [f32; 65536],
}

struct GameState {
    players: Vec<Player>,
    ground: Ground,
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

        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let b = &buf[..size];
                if let Some(mut m) = Message::try_from_data(peer, b) {
                    //println!("Received command {:?} from {}", m.command, m.socket_addr);
                    match m.command {
                        Command::BLOB => {},
                        Command::STATE => {
                            if let Some(pid) = m.extract_u8(0) {
                                //println!("PID {} wants the gamestate", pid);
                                let mut reply = Message::new(peer, Command::R_STATE);
                                reply.push_bytes((self.state.players.len() as u8).as_bytes());
                                reply.push_bytes(self.state.ground.num_mutations.as_bytes());
                                socket.send_to(&reply.get_bytes(), &peer).await?;
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        }
                        Command::POS => {
                            if let Some(pos) = m.extract_vec3a(1) {
                                //println!("PID {} is at {}", m.extract_u8(0).unwrap(), pos);
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        }
                        Command::MUT => {
                            if let Some(amt) = m.extract_f32(5) {
                                let pid = m.extract_u8(0).unwrap();
                                let idx = m.extract_u32(1).unwrap();
                                self.state.ground.num_mutations += 1;
                                self.state.ground.mutations[idx as usize] += amt;
                                println!("PID {} wants to mutate idx {} by {}", pid, idx, amt);

                                let stream = TcpStream::connect("127.0.0.1:42069").await?;
                                stream.writable().await?;

                                match stream.try_write(&m.get_bytes()) {
                                    Ok(n) => {
                                        println!("Sending {:?} bytes",n);
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    }
                                    Err(e) => {
                                    }
                                }

                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }

                        },
                        Command::PPOS => {
                            if let Some(qpid) = m.extract_u8(1) {
                                let pid = m.extract_u8(0).unwrap();
                                //println!("PID {} wants to know the position of PID {}", pid, qpid);
                                //let mut reply = Message::new(peer, Command::R_PPOS);
                                //reply.push_bytes(vec3a(0.0,0.0,0.0).as_bytes());
                                //socket.send_to(&reply.get_bytes(), &peer).await?;
                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        },
                        Command::GNDSTATE => {
                            if let Some(pid) = m.extract_u8(0) {
                                println!("PID {} wants to know the ground state", pid);
                                let mut reply = Message::new(peer, Command::R_GNDSTATE);
                                let size = 4*self.state.ground.mutations.len() as u32;
                                reply.push_bytes(size.as_bytes());
                                socket.send_to(&reply.get_bytes(), &peer).await?;

                                let stream = TcpStream::connect("127.0.0.1:42069").await?;
                                // Wait for the socket to be writable
                                stream.writable().await?;

                                // Try to write data, this may still fail with `WouldBlock`
                                // if the readiness event is a false positive.
                                let mut b = vec!();
                                for f in self.state.ground.mutations {
                                    b.append(&mut f.as_bytes());
                                }
                                match stream.try_write(b.as_slice()) {
                                    Ok(n) => {
                                        println!("Sending {:?} bytes",n);
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    }
                                    Err(e) => {
                                    }
                                }

                            } else {
                                println!("Invalid payload for command: {:?} (0x{:02x})", m.command, b[0]);
                            }
                        },
                        _ => {},
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
        .unwrap_or_else(|| "127.0.0.1:42069".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
        state: GameState {
            players: vec!(),
            ground: Ground {
                num_mutations: 0,
                mutations: [0f32; 65536],
            },
        },
    };

    // This starts the server task.
    server.run().await?;
    Ok(())
}
