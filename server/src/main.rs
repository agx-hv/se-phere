use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;
use strum_macros::FromRepr;
use glam::*;

#[derive(FromRepr, Debug, PartialEq)]
#[repr(u8)]
enum Commands {
    RESERVED,
    STATE,
    POS,
    MUT,
    PPOS,
}

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let _amt = socket.send_to(b"ACK", &peer).await?;

                let b = &buf[..size];
                let cmd = match b.get(0) {
                    Some(i) => Commands::from_repr(*i),
                    _ => None,
                };
                match cmd {
                    Some(Commands::POS) => {
                        match Self::vec3a_from_bytes(b.get(2..)) {
                            Some(v) => {
                                let pid = b[1];
                                println!("Received from {}: {:?} {} {}", &peer, cmd.unwrap(), pid, v);
                            }
                            _ => println!("Invalid Vector from {}", &peer),
                        }
                    },
                    _ => println!("Invalid Command from {}", &peer),
                }
                //println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
    fn vec3a_from_bytes(byte_array: Option<&[u8]>) -> Option<Vec3A> {
        match byte_array?.len() {
            12 => {
                let x = f32::from_be_bytes(byte_array?.get(..4)?.try_into().unwrap());
                let y = f32::from_be_bytes(byte_array?.get(4..8)?.try_into().unwrap());
                let z = f32::from_be_bytes(byte_array?.get(8..)?.try_into().unwrap());
                Some(vec3a(x,y,z))
            },
            _ => None,
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
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
