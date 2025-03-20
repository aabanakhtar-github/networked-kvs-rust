use crate::net_util::PacketBody::TextPacket;
use crate::net_util::PacketType::Hello;
use thiserror::Error;

const MIN_PACKET_LEN: usize = 5;

#[repr(u8)]
enum PacketType {
    Hello = 1,
}

enum PacketBody {
    TextPacket(String),
}


#[derive(Error, Debug)]
enum PacketError {
    #[error("Invalid packet type!")]
    InvalidPacketType,
    #[error("std error")]
    StdError(std::io::Error),
    #[error("Couldn't convert bytes to utf-8!")]
    StdUtf8Error(#[from] std::string::FromUtf8Error),
}

struct Packet {
    packet_type: PacketType,
    content_length: usize,
    content: PacketBody
}

impl PacketType {
    fn from_u8(packet_type: u8) -> Option<PacketType> {
        match packet_type {
            1 => Some(PacketType::Hello),
            _ => None
        }
    }
}

impl Packet {
    fn new(packet_type: PacketType, content_length: usize, content: PacketBody) -> Packet {
       Packet{packet_type, content_length, content}
    }

    fn from(bytes: Vec<u8>) -> Result<Packet, PacketError> {
        assert!(bytes.len() >= MIN_PACKET_LEN);
        // match the cases
        match PacketType::from_u8(bytes[0]) {
            Some(Hello) => {
                let mut res = Packet::new(Hello, 0, TextPacket("".to_string()));
                let len = u32::from_be_bytes(bytes[1..5].try_into()?) as usize;
                let &content_raw = &bytes[5..len];
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

    fn to_bytes(&self) -> Vec<u8> {
        let result = vec![0u8; self.content_length + MIN_PACKET_LEN];
        match &self.content {
            PacketBody::TextPacket(text) => {
                let mut sector = &result[MIN_PACKET_LEN..];
                self.buf_copy(&mut sector, &text.as_bytes().to_vec());
                result
            }
            _ => {vec![]}
        };
    }
}