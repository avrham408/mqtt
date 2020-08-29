#![allow(dead_code, unused_variables)]
use std::error::Error;

pub const CONNECT_BYTE: u8 = 0x10;
pub const CONNACK_BYTE: u8 = 0x20;
pub const PUBLISH_BYTE: u8 = 0x30;
pub const PUBACK_BYTE: u8 = 0x40;
pub const PUBREC_BYTE: u8 = 0x50;
pub const PUBREL_BYTE: u8 = 0x60;
pub const PUBCOMP_BYTE: u8 = 0x70;
pub const SUBSCRIBE_BYTE: u8 = 0x80;
pub const SUBACK_BYTE: u8 = 0x90;
pub const UNSUBSCRIBE_BYTE: u8 = 0xA0;
pub const UNSUBACK_BYTE: u8 = 0xB0;
pub const PINGREQ: u8 = 0xC0;
pub const PINGRESP_BYTE: u8 = 0xD0;
pub const DISCONNECT: u8 = 0xE0;
pub const AUTH: u8 = 0xF0;

pub const MQTT_HEADER_LEN: u8 = 2;
pub const MQTT_ACK_LEN: u8 = 2;
pub const MAX_LEN_BYTES_REMAINING_LENGTH: u8 = 4;

#[derive(Debug, PartialEq, Clone)]
pub enum QosLevel {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl QosLevel {
    pub fn new(qos: u8) -> Result<Self, Box<dyn Error>> {
        Ok(match qos {
            0 => Self::AtMostOnce,
            1 => Self::AtLeastOnce,
            2 => Self::ExactlyOnce,
            3 => return Err("parse qos failed 3 is not implmented".into()),
            _ => return Err("failed parsing qos the field is 2 bit field".into()),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeaderFlags {
    retain: bool,
    qos: QosLevel,
    dup: bool,
}

impl HeaderFlags {
    pub fn unpack(header_flag: [bool; 8]) -> Result<Self, Box<dyn Error>> {
        let qos = {
            let num: u8 = if header_flag[2] { 1 << 1 } else { 0 };
            if header_flag[3] {
                num + 1
            } else {
                num
            }
        };
        Ok(Self {
            retain: header_flag[0],
            qos: QosLevel::new(qos)?,
            dup: header_flag[3],
        })
    }

    pub fn pack(retain: bool, qos: QosLevel, dup: bool) -> Self {
        Self { retain, qos, dup }
    }
}

pub fn valid_all_data_parsed(buf: &Vec<u8>, pbuf: &mut usize) -> Result<(), Box<dyn Error>> {
    if *pbuf != buf.len() {
        return Err("Buffer has more data than parsed".into());
    }
    Ok(())
}
