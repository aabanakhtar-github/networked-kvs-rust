use std::error::Error;
use crate::packet::PacketBody::TextPacket;
use crate::packet::PacketType::Hello;
use thiserror::Error;
use std::convert::TryFrom;
use tokio_util::codec::{Encoder, Decoder};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufWriter, WriteHalf};
use tokio_util::bytes::BytesMut;

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
    #[error("Tokio Error!")]
    TokioError(#[from] tokio::io::Error),
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

        let mut header: [u8; 4] = bytes[1..5].iter().clone().as_slice().try_into().unwrap();
        let len = u32::from_be_bytes(header) as usize;

        if (len > bytes.len() - MIN_PACKET_LEN) {
            return Err(PacketError::InvalidPacketType)
        }

        let content_raw = &bytes[MIN_PACKET_LEN..MIN_PACKET_LEN + len];
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
    fn buf_copy(&self, buf: &mut [u8], from: &[u8]) {
        let min_len = buf.len().min(from.len());
        buf[..min_len].copy_from_slice(&from[..min_len]);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![0u8; self.content_length + MIN_PACKET_LEN];
        result[0] = self.packet_type.to_u8();
        let header_length = u32::to_be_bytes(self.content_length as u32).to_vec();
        result[1..5].copy_from_slice(&header_length);
        match &self.content {
            PacketBody::TextPacket(text) => {
                self.buf_copy(&mut result[MIN_PACKET_LEN..], &text.as_bytes().to_vec());
                result
            }
            _ => {vec![0u8; 5]}
        }
    }
}

impl Encoder<Packet> for Packet {
    type Error = PacketError;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Decoder for Packet {
    type Item = Packet;
    type Error = PacketError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}