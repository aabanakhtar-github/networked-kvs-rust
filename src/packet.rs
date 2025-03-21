use std::error::Error;
use crate::packet::PacketBody::TextPacket;
use crate::packet::PacketType::Hello;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufWriter, WriteHalf};

pub const MIN_PACKET_LEN: usize = 5;

#[repr(u8)]
pub enum PacketType {
    Hello = 1,
}

pub enum PacketBody {
    TextPacket(String),
}


#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Invalid packet type!")]
    InvalidPacketType,
    #[error("std error")]
    StdError(std::io::Error),
    #[error("Couldn't convert bytes to utf-8!")]
    StdUtf8Error(#[from] std::string::FromUtf8Error),
}

pub struct Packet {
    pub(crate) packet_type: PacketType,
    pub(crate) content_length: usize,
    pub(crate) content: PacketBody
}

impl PacketType {
    pub fn from_u8(packet_type: u8) -> Option<PacketType> {
        match packet_type {
            1 => Some(PacketType::Hello),
            _ => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        match &self {
            PacketType::Hello => 1,
        }
    }
}

impl Packet {
    pub fn new(packet_type: PacketType, content_length: usize, content: PacketBody) -> Packet {
       Packet{packet_type, content_length, content}
    }

    pub fn from(bytes: Vec<u8>) -> Result<Packet, PacketError> {
        if (!bytes.len() > MIN_PACKET_LEN) {
           return Err(PacketError::InvalidPacketType)
        }

        let mut vec = bytes[1..5]?;
        let len = u32::from_be_bytes(vec) as usize;

        if (len > bytes.len() - MIN_PACKET_LEN) {
            return Err(PacketError::InvalidPacketType)
        }

        let &content_raw = &bytes[MIN_PACKET_LEN..MIN_PACKET_LEN + len];
        // match the cases
        match PacketType::from_u8(bytes[0]) {
            Some(Hello) => {
                let mut res = Packet::new(Hello, 0, TextPacket("".to_string()));
                let packet_contents = content_raw.to_vec();
                let packet_contents_str = String::from_utf8(packet_contents)?;
                Ok(res)
            },
            None => Err(PacketError::InvalidPacketType),
        }
    }

    // utility function
    fn buf_copy(buf: &mut [u8], from: &[u8]) {
        let min_len = buf.len().min(from.len());
        buf[..min_len].copy_from_slice(&from[..min_len]);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let result = vec![0u8; self.content_length + MIN_PACKET_LEN];
        result[0] = self.packet_type.to_u8();
        result[1..5] = u32::to_be_bytes(self.content_length as u32).try_into().unwrap();
        match &self.content {
            PacketBody::TextPacket(text) => {
                let mut sector = &result[MIN_PACKET_LEN..];
                self.buf_copy(&mut sector, &text.as_bytes().to_vec());
                result
            }
            _ => {vec![]}
        }
    }

    pub async fn send() -> Result<(), PacketError> {
       Ok(())
    }
}
