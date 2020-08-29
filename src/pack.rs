#![allow(unused)] //this line must delete on prd
use std::error::Error;

pub fn unpack_u8(buf: &Vec<u8>, pbuf: &mut usize) -> Result<u8, Box<dyn Error>> {
    let val = buf.get(*pbuf);
    *pbuf += 1;
    match val {
        Some(num) => Ok(num.clone()),
        None => Err(format!("Index error to vec {}", *pbuf - 1).into()),
    }
}

pub fn unpack_u16(buf: &Vec<u8>, pbuf: &mut usize) -> Result<u16, Box<dyn Error>> {
    let split_val = &buf.get(*pbuf..*pbuf + 2);
    *pbuf += 2;
    match split_val {
        Some(val) => Ok((val[0] as u16) << 8 | val[1] as u16),
        None => Err(format!("Index error to vec {}", *pbuf - 2).into()),
    }
}

pub fn unpack_u32(buf: &Vec<u8>, pbuf: &mut usize) -> Result<u32, Box<dyn Error>> {
    let val1 = unpack_u16(buf, pbuf)?;
    let val2 = unpack_u16(buf, pbuf)?;
    Ok((val1 as u32) << 16 | val2 as u32)
}

pub fn unpack_bytes(
    buf: &Vec<u8>,
    pbuf: &mut usize,
    size: usize,
) -> Result<String, Box<dyn Error>> {
    // think for future how we want it maybe not String
    let bytes = buf.get(*pbuf..*pbuf + size);
    match bytes {
        Some(val) => {
            let st = String::from_utf8(val.to_vec())?.to_string();
            *pbuf += size;
            return Ok(st);
        }
        None => return Err(format!("Index error to vec {} to {} ", *pbuf, *pbuf + size).into()),
    }
}

pub fn unpack_vec(buf: &Vec<u8>, pbuf: &mut usize, size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = buf.get(*pbuf..*pbuf + size);
    match bytes {
        Some(val) => {
            *pbuf += size;
            return Ok(val.to_vec());
        }
        None => return Err(format!("Index error to vec {} to {} ", *pbuf, *pbuf + size).into()),
    }
}

pub fn unpack_byte(num: u8) -> [bool; 8] {
    let mut arr = [false; 8];
    for (pos, i) in (0..8).enumerate() {
        let a = (num & (1 << i)) >> i;
        arr[pos] = a == 1;
    }
    arr
}

pub fn unpack_string(buf: &Vec<u8>, pbuf: &mut usize) -> Result<String, Box<dyn Error>> {
    let str_len = unpack_u16(buf, pbuf)?;
    Ok(unpack_bytes(buf, pbuf, str_len as usize)?)
}

pub fn pack_u8(buf: &mut Vec<u8>, val: u8) {
    buf.push(val);
}

pub fn pack_u16(buf: &mut Vec<u8>, val: u16) {
    pack_u8(buf, (val >> 8) as u8);
    pack_u8(buf, val as u8);
}

pub fn pack_u32(buf: &mut Vec<u8>, val: u32) {
    pack_u16(buf, (val >> 16) as u16);
    pack_u16(buf, val as u16);
}

pub fn pack_bytes() {
    panic!("NOT IMPLMENTED -> add support for future pack bytes");
}

mod test {
    use super::*;

    #[test]
    fn test_unpack_u8() {
        let buf: Vec<u8> = vec![1, 3, 4, 2, 1];
        let mut pointer: usize = 0;
        assert_eq!(1, unpack_u8(&buf, &mut pointer).unwrap());
        assert_eq!(3, unpack_u8(&buf, &mut pointer).unwrap());
        assert_eq!(4, unpack_u8(&buf, &mut pointer).unwrap());
        assert_eq!(2, unpack_u8(&buf, &mut pointer).unwrap());
        assert_eq!(1, unpack_u8(&buf, &mut pointer).unwrap());
    }

    #[test]
    fn test_unpack_u8_negative() {
        let buf: Vec<u8> = vec![1];
        let mut pointer: usize = 0;
        assert_eq!(1, unpack_u8(&buf, &mut pointer).unwrap());
        assert!(unpack_u8(&buf, &mut pointer).is_err());
    }

    #[test]
    fn test_unpack_u16() {
        let buf: Vec<u8> = vec![40, 30, 120, 39, 11, 75];
        let mut pointer: usize = 0;
        assert_eq!(10270, unpack_u16(&buf, &mut pointer).unwrap());
        assert_eq!(pointer, 2);
        assert_eq!(30759, unpack_u16(&buf, &mut pointer).unwrap());
        assert_eq!(pointer, 4);
        assert_eq!(2891, unpack_u16(&buf, &mut pointer).unwrap());
        assert_eq!(pointer, 6);
    }

    #[test]
    fn test_unpack_u16_negative() {
        let buf: Vec<u8> = vec![1];
        let mut pointer: usize = 0;
        assert!(unpack_u16(&buf, &mut pointer).is_err());
    }

    #[test]
    fn test_unpack_u32() {
        let buf: Vec<u8> = vec![13, 2, 2, 1, 11, 75];
        let mut pointer: usize = 0;
        let val = unpack_u32(&buf, &mut pointer).unwrap();
        assert_eq!(val, 218235393);
        assert_eq!(pointer, 4);
    }

    #[test]
    fn test_unpack_u32_negative() {
        let buf: Vec<u8> = vec![1, 2, 3];
        let mut pointer: usize = 0;
        assert!(unpack_u32(&buf, &mut pointer).is_err());
    }

    #[test]
    fn test_unpack_bytes() {
        let buf: Vec<u8> = vec![65, 86, 105, 1, 11, 75];
        let mut pointer: usize = 0;
        let st = unpack_bytes(&buf, &mut pointer, 3).unwrap();
        assert_eq!("AVi".to_string(), st);
        assert_eq!(pointer, st.len());
        assert!(unpack_bytes(&buf, &mut pointer, 5).is_err());
    }

    #[test]
    fn test_pack_u8() {
        let mut buf: Vec<u8> = vec![65, 86, 105, 1, 11, 75];
        pack_u8(&mut buf, 5);
        assert_eq!(buf.len(), 7);
        assert_eq!(buf[6], 5);
    }

    #[test]
    fn test_pack_u16() {
        let mut buf: Vec<u8> = vec![65];
        pack_u16(&mut buf, 600);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2] as u16, 600 - (256 * 2));
        pack_u16(&mut buf, 100);
        assert_eq!(buf[3], 0);
        assert_eq!(buf[4], 100);
    }

    #[test]
    fn test_pack_u32() {
        let mut buf: Vec<u8> = vec![];
        pack_u32(&mut buf, 600);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[1], 0);
        assert_eq!(buf[2] as u16, 2);
        assert_eq!(buf[3] as u16, 600 - (256 * 2));
        pack_u32(&mut buf, 16843009);
        assert_eq!(buf[4], 1);
        assert_eq!(buf[5], 1);
        assert_eq!(buf[6], 1);
        assert_eq!(buf[7], 1);
    }

    #[test]
    fn test_unpack_byte() {
        let l = unpack_byte(1);
        assert_eq!(l, [true, false, false, false, false, false, false, false]);
        let l = unpack_byte(3);
        assert_eq!(l, [true, true, false, false, false, false, false, false]);
        let l = unpack_byte(255);
        assert_eq!(l, [true, true, true, true, true, true, true, true]);
        let l = unpack_byte(124);
        assert_eq!(l, [false, false, true, true, true, true, true, false]);
    }
}
