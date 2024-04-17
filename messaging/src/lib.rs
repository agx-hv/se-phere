use strum_macros::FromRepr;
use std::net::SocketAddr;
use glam::*;

#[derive(FromRepr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    BLOB,              // 0x00
    STATE,             // 0x01
    POS,               // 0x02
    MUT,               // 0x03
    R_STATE,           // 0x04
    PPOS,              // 0x05
    R_PPOS,            // 0x06
    GNDSTATE,          // 0x07
    R_GNDSTATE,        // 0x08
    LOGIN,             // 0x09
    SET_PID            // 0x10
}

#[derive(Debug)]
pub struct Message {
    pub command: Command,
    pub payload: Vec<u8>,
}

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for Vec3A {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = vec!();
        let (x,y,z) = (self.x, self.y, self.z);
        result.extend_from_slice(&x.to_be_bytes());
        result.extend_from_slice(&y.to_be_bytes());
        result.extend_from_slice(&z.to_be_bytes());
        result
    }
}

impl AsBytes for f32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsBytes for u64 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsBytes for u32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl AsBytes for u8 {
    fn as_bytes(&self) -> Vec<u8> {
        vec!(*self)
    }
}


impl Message {
    pub fn new(command: Command) -> Self {
        Message {
            command,
            payload: vec!(),
        }
    }
    pub fn push_bytes(&mut self, mut bytes: Vec<u8>) {
        self.payload.append(&mut bytes);
    }
    pub fn get_bytes(&mut self) -> Vec<u8> {
        let mut bytes = vec!(self.command as u8);
        bytes.append(&mut self.payload);
        bytes
    }
    pub fn try_from_data(_socket_addr: SocketAddr, data: &[u8]) -> Option<Self> {
        let command = Command::from_repr(*data.get(0)?);
        let mut payload = vec!();
        payload.extend_from_slice(data.get(1..)?);
        Some(Message {
            command: command?,
            payload: payload,
        })
    }
    pub fn extract_u8(&self, offset: usize) -> Option<u8> {
        if offset < self.payload.len() {
            return Some(self.payload[offset]);
        }
        None
    }
    pub fn extract_u32(&self, offset: usize) -> Option<u32> {
        if offset+4 <= self.payload.len() {
            return Some(u32::from_be_bytes(self.payload[offset..offset+4].try_into().unwrap()));
        }
        None
    }
    pub fn extract_u64(&self, offset: usize) -> Option<u64> {
        if offset+8 <= self.payload.len() {
            return Some(u64::from_be_bytes(self.payload[offset..offset+8].try_into().unwrap()));
        }
        None
    }
    pub fn extract_f32(&self, offset: usize) -> Option<f32> {
        if offset+4 <= self.payload.len() {
            return Some(f32::from_be_bytes(self.payload[offset..offset+4].try_into().unwrap()));
        }
        None
    }
    pub fn extract_vec3a(&self, offset: usize) -> Option<Vec3A> {
        if offset+12 <= self.payload.len() {
            let x = f32::from_be_bytes(self.payload[offset..offset+4].try_into().unwrap());
            let y = f32::from_be_bytes(self.payload[offset+4..offset+8].try_into().unwrap());
            let z = f32::from_be_bytes(self.payload[offset+8..offset+12].try_into().unwrap());
            return Some(vec3a(x,y,z));
        }
        None
    }
}
