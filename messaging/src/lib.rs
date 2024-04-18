use glam::*;
use std::net::SocketAddr;
use strum_macros::FromRepr;

// Enum to represent command types
#[derive(FromRepr, Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    BLOB,      // 0x00
    STATE,     // 0x01
    POS,       // 0x02
    MUT,       // 0x03
    RSTATE,    // 0x04
    PPOS,      // 0x05
    RPPOS,     // 0x06
    GNDSTATE,  // 0x07
    RGNDSTATE, // 0x08
    LOGIN,     // 0x09
    SETPID,    // 0x0A
}

// Message struct storing command and payload
#[derive(Debug)]
pub struct Message {
    pub command: Command,
    pub payload: Vec<u8>,
}

// Trait that allows byte representations of data types
pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

// Trait implementations to convert primitives and vector types into byte representations
impl AsBytes for Vec3A {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        let (x, y, z) = (self.x, self.y, self.z);
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
        vec![*self]
    }
}

// Message constructor and methods
impl Message {
    
    // Message default constructor
    pub fn new(command: Command) -> Self {
        Message {
            command,
            payload: vec![],
        }
    }

    // Method to push a vector of bytes into the payload vector
    pub fn push_bytes(&mut self, mut bytes: Vec<u8>) {
        self.payload.append(&mut bytes);
    }

    // Method to retrieve bytes (payload & command) to be sent over network
    pub fn get_bytes(&mut self) -> Vec<u8> {
        let mut bytes = vec![self.command as u8];
        bytes.append(&mut self.payload);
        bytes
    }

    // Attempt to create a message instance from bytes, special constructor
    // Returns Some(Message) if bytes are valid
    // Otherwise returns None
    pub fn try_from_data(_socket_addr: SocketAddr, data: &[u8]) -> Option<Self> {
        let command = Command::from_repr(*data.get(0)?);
        let mut payload = vec![];
        payload.extend_from_slice(data.get(1..)?);
        Some(Message {
            command: command?,
            payload: payload,
        })
    }

    // Attempts to extract one byte at a specified index
    // Returns Some(u8) if index is accessible
    // Otherwise returns None
    pub fn extract_u8(&self, offset: usize) -> Option<u8> {
        if offset < self.payload.len() {
            return Some(self.payload[offset]);
        }
        None
    }

    // Attempts to extract 4 bytes as a u32 at a specified index
    // Returns Some(u32) if index is accessible
    // Otherwise returns None
    pub fn extract_u32(&self, offset: usize) -> Option<u32> {
        if offset + 4 <= self.payload.len() {
            return Some(u32::from_be_bytes(
                self.payload[offset..offset + 4].try_into().unwrap(),
            ));
        }
        None
    }

    // Attempts to extract 8 bytes as a u64 at a specified index
    // Returns Some(u64) if index is accessible
    // Otherwise returns None
    pub fn extract_u64(&self, offset: usize) -> Option<u64> {
        if offset + 8 <= self.payload.len() {
            return Some(u64::from_be_bytes(
                self.payload[offset..offset + 8].try_into().unwrap(),
            ));
        }
        None
    }

    // Attempts to extract 4 bytes as a f32 at a specified index
    // Returns Some(f32) if index is accessible
    // Otherwise returns None
    pub fn extract_f32(&self, offset: usize) -> Option<f32> {
        if offset + 4 <= self.payload.len() {
            return Some(f32::from_be_bytes(
                self.payload[offset..offset + 4].try_into().unwrap(),
            ));
        }
        None
    }

    // Attempts to extract 12 bytes as a Vec3A<f32> at a specified index
    // Returns Some(Vec3A<f32>) if index is accessible
    // Otherwise returns None
    pub fn extract_vec3a(&self, offset: usize) -> Option<Vec3A> {
        if offset + 12 <= self.payload.len() {
            let x = f32::from_be_bytes(self.payload[offset..offset + 4].try_into().unwrap());
            let y = f32::from_be_bytes(self.payload[offset + 4..offset + 8].try_into().unwrap());
            let z = f32::from_be_bytes(self.payload[offset + 8..offset + 12].try_into().unwrap());
            return Some(vec3a(x, y, z));
        }
        None
    }
}
