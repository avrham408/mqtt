#![allow(dead_code, unused_variables, non_snake_case)] //this line must delete on prd use crate::pack::*;
use crate::pack::*;
use crate::protocol_util::*;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Packet {
    Connect(Connect),
    Connnack(Connack),
    Publish(Publish),
    Puback(Puback),
    Pubrec(Pubrec),
    Pubrel(Pubrel),
    Pubcomp(Pubcomp),
    Subscribe(Subscribe),
    Suback(Suback),
    Unsubscribe(Unsubscribe),
    Unsuback(Unsuback),
    Pingreq(Pingreq),
    Pingresp(Pingresp),
    Disconnect(Disconnect),
    Auth(Auth),
}

trait Message {
    // class method read message to packet type
    fn parse(buf: &Vec<u8>, header_flags: HeaderFlags) -> Result<Box<Self>, Box<dyn Error>>;

    fn send_bytes(&self) -> Vec<u8>;
}

pub mod protocol {
    use super::*;
    use std::error::Error;

    pub fn parse_packet(buf: &Vec<u8>) -> Result<Packet, Box<dyn Error>> {
        let (packet_type, header_flags) = split_to_message_type_and_header_flags(buf[0])?;
        let packet = match packet_type {
            CONNECT_BYTE => Ok(Packet::Connect(*Connect::parse(buf, header_flags)?)),
            CONNACK_BYTE => Ok(Packet::Connack(*Connack::parse(buf, header_flags)?)),
            _ => return Err(format!("message type unsupported - 0X{:x}", packet_type).into()),
        };
        packet
    }

    fn split_to_message_type_and_header_flags(
        bit: u8,
    ) -> Result<(u8, HeaderFlags), Box<dyn Error>> {
        let header_flag = unpack_byte(bit & 0x0F);
        let header_flags = HeaderFlags::unpack(header_flag)?;
        Ok((bit & 0xf0, header_flags))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connect {
    pub protocol_level: u8,
    pub headerflags: HeaderFlags,
    pub reserved: bool,
    pub clean_session: bool,
    pub will_qos: u8,
    pub will_retain: bool,
    pub keep_alive: u16,
    pub username: Option<String>,
    pub client_id: String,
    pub password: Option<String>,
    pub will_topic: Option<String>,
    pub will_message: Option<String>,
}

impl Connect {
    fn new(
        protocol_level: u8,
        headerflags: HeaderFlags,
        reserved: bool,
        clean_session: bool,
        will_qos: u8,
        will_retain: bool,
        keep_alive: u16,
        username: Option<String>,
        client_id: String,
        password: Option<String>,
        will_topic: Option<String>,
        will_message: Option<String>,
    ) -> Self {
        Self {
            protocol_level,
            headerflags,
            reserved,
            clean_session,
            will_qos,
            will_retain,
            keep_alive,
            client_id,
            username,
            password,
            will_topic,
            will_message,
        }
    }

    fn parse_will_qos(byte: [bool; 8]) -> u8 {
        let wil_qos: u8 = {
            let num: u8 = if byte[3] { 1 << 1 } else { 0 };
            if byte[4] {
                num + 1
            } else {
                num
            }
        };
        wil_qos
    }

    fn unpack_header(byte: u8) -> (bool, bool, bool, u8, bool, bool, bool) {
        let byte_split = unpack_byte(byte);
        let will_qos = Self::parse_will_qos(byte_split);
        (
            byte_split[0], // reserved
            byte_split[1], // clean_session
            byte_split[2], // will
            will_qos,
            byte_split[5], //will retain
            byte_split[6], // password
            byte_split[7], // username
        )
    }

    fn unpack(
        buf: &Vec<u8>,
        pbuf: &mut usize,
        headerflags: HeaderFlags,
    ) -> Result<Self, Box<dyn Error>> {
        let protocol_level = unpack_u8(buf, pbuf)?;
        let connect_flags = unpack_u8(buf, pbuf)?;
        let (reserved, clean_session, will, will_qos, will_retain, password_flag, username_flag) =
            Self::unpack_header(connect_flags);
        let keep_alive = unpack_u16(buf, pbuf)?;

        let will_topic = Option::None;
        let will_message = Option::None;
        let username = Option::None;
        let password = Option::None;

        let client_id = unpack_string(buf, pbuf)?;

        if will {
            let will_topic = Option::Some(unpack_string(buf, pbuf)?);
            let will_message = Option::Some(unpack_string(buf, pbuf)?);
        }
        if username_flag {
            let username = Option::Some(unpack_string(buf, pbuf)?);
        }
        if password_flag {
            let password = Option::Some(unpack_string(buf, pbuf)?);
        }

        valid_all_data_parsed(buf, pbuf)?;

        Ok(Self::new(
            protocol_level,
            headerflags,
            reserved,
            clean_session,
            will_qos,
            will_retain,
            keep_alive,
            username,
            client_id,
            password,
            will_topic,
            will_message,
        ))
    }
}

impl Message for Connect {
    fn parse(buf: &Vec<u8>, header_flags: HeaderFlags) -> Result<Box<Self>, Box<dyn Error>> {
        let mut start_pos = 8;
        return Ok(Box::new(Self::unpack(buf, &mut start_pos, header_flags)?));
    }

    fn send_bytes(&self) -> Vec<u8> {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connack {
    headerflags: HeaderFlags,
    sp: bool,
    rc: u16,
}

impl Connack {
    fn new(headerflags: HeaderFlags, sp: bool, rc: u16) -> Self {
        Self {
            headerflags,
            sp,
            rc,
        }
    }

    fn unpack(
        buf: &Vec<u8>,
        pbuf: &mut usize,
        headerflags: HeaderFlags,
    ) -> Result<Self, Box<dyn Error>> {
        todo!();
    }
}

impl Message for Connack {
    fn parse(buf: &Vec<u8>, header_flags: HeaderFlags) -> Result<Box<Self>, Box<dyn Error>> {
        let start_pos = 1;
        return Ok(Box::new(Self::unpack(buf, &mut start_pos, header_flags)?));
    }

    fn send_bytes(&self) -> Vec<u8> {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Publish {}

impl Publish {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Puback {}

impl Puback {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pubrec {}

impl Pubrec {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pubrel {}

impl Pubrel {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pubcomp {}

impl Pubcomp {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Subscribe {}

impl Subscribe {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Suback {}

impl Suback {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unsubscribe {}

impl Unsubscribe {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unsuback {}

impl Unsuback {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pingreq {}

impl Pingreq {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pingresp {}

impl Pingresp {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Disconnect {}

impl Disconnect {
    fn new() {
        todo!();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Auth {}

impl Auth {
    fn new() {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_parse_connect_not_fields() {
        let mut f = fs::File::open("./test_data/clean_connect").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        match parse_packet(&buf).unwrap() {
            Packet::Connect(pack) => {
                assert_eq!(pack.protocol_level, 4);
                assert_eq!(pack.reserved, false);
                assert_eq!(pack.clean_session, true);
                assert_eq!(pack.will_qos, 0);
                assert_eq!(pack.will_retain, false);
                assert_eq!(pack.keep_alive, 60);
                assert_eq!(pack.username, Option::None);
                assert_eq!(pack.password, Option::None);
                assert_eq!(pack.client_id, "__main__".to_string());
                assert_eq!(pack.password, Option::None);
                assert_eq!(pack.will_topic, Option::None);
                assert_eq!(pack.will_message, Option::None);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_connect_not_client_name() {
        let mut f = fs::File::open("./test_data/connect_not_cid").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        parse_packet(&buf).unwrap();
    }

    #[test]
    fn test_parse_bad_connect() {
        let mut f = fs::File::open("./test_data/bad_connect").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        assert!(parse_packet(&buf).is_err());
    }

    #[test]
    fn test_parse_extra_data_in_connect() {
        let mut f = fs::File::open("./test_data/extra_data_in_connect").unwrap();
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        assert!(parse_packet(&buf).is_err());
    }
}
