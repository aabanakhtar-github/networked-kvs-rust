use thiserror::Error;
use std::default::Default;
use std::convert::{TryFrom, TryInto};
use tokio_util::codec::{Encoder, Decoder};
use tokio_util::bytes::{Buf, BufMut, BytesMut};

pub const MIN_PACKET_LEN: usize = 5;

#[repr(u8)]
#[derive(Default)]
pub enum PacketType {
    #[default]
    TextPacket = 1,
    GetRequest = 2,
    SetRequest = 3,
    DelRequest = 4,
    PingRequest = 5
}

pub enum PacketBody {
    TextPacket(String),
    RequestBody{
        key: String,
        new_value: Option<String>
    }
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Generic Error {0}")]
    GenericError(String),
    #[error("Invalid packet type!")]
    InvalidPacketType,
    #[error("std error")]
    StdError(std::io::Error),
    #[error("Couldn't convert bytes to utf-8!")]
    StdUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Tokio Error!")]
    TokioError(#[from] tokio::io::Error),
}

#[derive(Default)]
pub struct Packet {
    pub packet_type: PacketType,
    pub content_length: usize,
    pub content: PacketBody
}

impl Default for PacketBody {
    fn default() -> Self { PacketBody::TextPacket(String::default()) }
}

impl PacketType {
    pub fn from_u8(packet_type: u8) -> Option<PacketType> {
        match packet_type {
            1 => Some(PacketType::TextPacket),
            2 => Some(PacketType::GetRequest),
            3 => Some(PacketType::SetRequest),
            4 => Some(PacketType::DelRequest),
            5 => Some(PacketType::PingRequest),
            _ => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        match &self {
            PacketType::TextPacket => 1,
            PacketType::GetRequest => 2,
            PacketType::SetRequest => 3,
            PacketType::DelRequest => 4,
            PacketType::PingRequest => 5,
        }
    }
}

impl Packet {
    pub fn new(packet_type: PacketType, content_length: usize, content: PacketBody) -> Packet {
       Packet{packet_type, content_length, content}
    }

    // utility function
    fn buf_copy(&self, buf: &mut [u8], from: &[u8]) {
        let min_len = buf.len().min(from.len());
        buf[..min_len].copy_from_slice(&from[..min_len]);
    }
}

pub struct PacketCodec;

impl Encoder<Packet> for PacketCodec {
    type Error = PacketError;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.content_length + MIN_PACKET_LEN);
        dst.put_u8(item.packet_type.to_u8());
        let len_bytes: [u8; 4] = (item.content_length as u32).to_be_bytes();
        dst.put_slice(&len_bytes);
        let content = match &item.content {
            PacketBody::TextPacket(value) => value.as_bytes()
        };
        dst.put_slice(content);
        Ok(())
    }
}

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = PacketError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < MIN_PACKET_LEN {
            return Ok(None)
        }

        let p_type = match PacketType::from_u8(src[0]) {
           Some(p_type) => p_type,
            None => return Err(PacketError::InvalidPacketType),
        };
        let p_header: [u8; 4] = src[1..5].try_into()
            .map_err(|_| PacketError::GenericError("Failed to parse packet header".to_string()))?;
        let p_content_len = u32::from_be_bytes(p_header) as usize;

        if src.len() < MIN_PACKET_LEN + p_content_len {
            return Ok(None)
        }

        let mut result: Packet = Default::default();
        result.packet_type = p_type;
        result.content = match &result.packet_type {
            PacketType::TextPacket => PacketBody::TextPacket(String::from_utf8(src[MIN_PACKET_LEN..].to_vec())?),
            PacketType::DelRequest | PacketType::GetRequest | PacketType::SetRequest => {

            },
        };
        result.content_length = p_content_len;

        src.advance(MIN_PACKET_LEN + p_content_len);
        Ok(Some(result))
    }
}