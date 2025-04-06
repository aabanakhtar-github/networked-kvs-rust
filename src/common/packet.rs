use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::usize::MIN;
use thiserror::Error;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use crate::common::packet::PacketBody::EmptyBody;
use crate::common::util::ByteSize;

pub const MIN_PACKET_LEN: usize = 5;

#[repr(u8)]
#[derive(Default, Debug, Clone)]
pub enum PacketType {
    #[default]
    TextPacket = 1,
    GetRequest = 2,
    SetRequest = 3,
    DelRequest = 4,
    PingRequest = 5,
}

#[derive(Default, Clone)]
#[derive(Debug)]
pub enum PacketBody {
    #[default]
    EmptyBody,
    TextPacket(String),
    RequestBody {
        key: String,
        new_value: Option<String>,
    },
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

#[derive(Default, Clone)]
pub struct Packet {
    pub packet_type: PacketType,
    pub content: PacketBody,
}

impl ByteSize for PacketBody {
    fn byte_size(&self) -> usize {
        match self {
            PacketBody::TextPacket(v) => v.len(),
            PacketBody::RequestBody {key, new_value: Some(v)} => {
                // strings + two 4 byte blocks with length data
                key.len() + v.len() + 4 + 4
            }, 
            PacketBody::EmptyBody => 0,
            PacketBody::RequestBody { key, ..} => {
                // similar principle
                key.len() + 4 
            }
        }
    }
}

impl PacketType {
    pub fn from_u8(packet_type: u8) -> Option<PacketType> {
        match packet_type {
            1 => Some(PacketType::TextPacket),
            2 => Some(PacketType::GetRequest),
            3 => Some(PacketType::SetRequest),
            4 => Some(PacketType::DelRequest),
            5 => Some(PacketType::PingRequest),
            _ => None,
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
    pub fn new(packet_type: PacketType, content: PacketBody) -> Packet {
        Packet {
            packet_type,
            content,
        }
    }
}

pub struct PacketCodec;

impl Encoder<Packet> for PacketCodec {
    type Error = PacketError;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(item.packet_type.to_u8());
        println!("{}", item.content.byte_size());
        dst.put_u32(item.content.byte_size() as u32);

        let mut tmp_buf = Vec::new();
        let content = match &item.content {
            PacketBody::TextPacket(value) => value.as_bytes(),
            PacketBody::RequestBody { key, new_value } => {
                tmp_buf.put_u32(key.len() as u32);
                tmp_buf.put_slice(key.as_bytes());
                
                if let Some(value) = new_value {
                    tmp_buf.put_u32(value.len() as u32); 
                    tmp_buf.put_slice(value.as_bytes());
                }

                tmp_buf.as_slice()
            },
            PacketBody::EmptyBody => &[] as &[u8]
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
            return Ok(None);
        }

        let packet_type = match PacketType::from_u8(src[0]) {
            Some(packet_type) => packet_type,
            None => return Err(PacketError::InvalidPacketType),
        };
        
        println!("Packet type: {:?}", packet_type);

        let content_len = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;
        
        println!("Content len: {}", content_len); 
        
        if src.len() < MIN_PACKET_LEN + content_len {
            return Ok(None);
        }

        src.advance(5);
        let content_bytes = src.split_to(content_len).to_vec();

        let packet_body = match packet_type {
            PacketType::TextPacket => {
                PacketBody::TextPacket(
                    String::from_utf8(content_bytes)
                        .map_err(PacketError::StdUtf8Error)?
                )
            },
            PacketType::DelRequest | PacketType::GetRequest | PacketType::SetRequest => {
                let key_len = u32::from_be_bytes([
                    content_bytes[0],
                    content_bytes[1],
                    content_bytes[2],
                    content_bytes[3],
                ]) as usize;
                let key = String::from_utf8(content_bytes[4..4 + key_len].to_vec())?;
                let new_value =  if content_bytes.len() > 4 + key_len {
                    let offset = 4 + key_len;
                    let value_key_bytes = &content_bytes[offset..offset + 4];
                    let value_len = u32::from_be_bytes([
                        value_key_bytes[0],
                        value_key_bytes[1],
                        value_key_bytes[2],
                        value_key_bytes[3],
                    ]) as usize;
                    let new_value =
                        String::from_utf8(content_bytes[offset + 4..offset+ 4 + value_len].to_vec())?;

                    Some(new_value)
                } else {
                    None
                };

                PacketBody::RequestBody {
                    key,
                    new_value
                }
            },
            PacketType::PingRequest => EmptyBody
        };
        
        Ok(Some(Packet {
            packet_type,
            content: packet_body,
        }))
    }

}