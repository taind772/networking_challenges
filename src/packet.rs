use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    bytes_to_u32,
    constants::{
        DATA_OFFSET, HEADER_SIZE, LEN_END, LEN_OFFSET, PKT_BYE, PKT_CALC, PKT_FLAG, PKT_HELLO,
        PKT_RESULT, TYPE_END, TYPE_OFFSET,
    },
};

pub enum PacketType {
    PktHello,
    PktCalc,
    PktResult,
    PktBye,
    PktFlag,
}

impl PacketType {
    pub fn to_id(&self) -> u32 {
        match self {
            Self::PktHello => PKT_HELLO,
            Self::PktCalc => PKT_CALC,
            Self::PktResult => PKT_RESULT,
            Self::PktBye => PKT_BYE,
            Self::PktFlag => PKT_FLAG,
        }
    }
    pub fn from_id(id: u32) -> Result<Self, &'static str> {
        match id {
            PKT_HELLO => Ok(Self::PktHello),
            PKT_CALC => Ok(Self::PktCalc),
            PKT_RESULT => Ok(Self::PktResult),
            PKT_BYE => Ok(Self::PktBye),
            PKT_FLAG => Ok(Self::PktFlag),
            _ => Err("Unknown package type id"),
        }
    }
}
pub struct Packet {
    bytes: Vec<u8>,
}

impl Packet {
    pub fn new(kind: PacketType, data: Option<&[u8]>) -> Self {
        use std::convert::TryFrom;
        let data = data.unwrap_or(&[]);
        let len = u32::try_from(data.len()).unwrap();

        let mut bytes = Vec::from(kind.to_id().to_le_bytes());
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(data);

        Self { bytes }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: Vec::from(bytes),
        }
    }

    pub fn kind(&self) -> PacketType {
        PacketType::from_id(bytes_to_u32(&self.bytes[TYPE_OFFSET..TYPE_END])).unwrap()
    }

    pub fn data_len(&self) -> u32 {
        bytes_to_u32(&self.bytes[LEN_OFFSET..LEN_END])
    }

    pub fn data(&self) -> Option<&[u8]> {
        if self.bytes.len() <= HEADER_SIZE {
            None
        } else {
            Some(&self.bytes[DATA_OFFSET..])
        }
    }

    pub fn read(stream: &mut TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = [0; HEADER_SIZE];
        stream.read_exact(&mut buf)?;
        let mut pkt = Self::from_bytes(&buf);
        if pkt.data_len() > 0 {
            let mut buf = vec![0; pkt.data_len() as usize];
            stream.read_exact(&mut buf)?;
            pkt.bytes.append(&mut buf);
        };
        Ok(pkt)
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<usize, std::io::Error> {
        stream.write(&self.bytes)
    }
}
