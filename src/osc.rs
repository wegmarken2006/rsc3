pub fn encode_i8(num: i32) -> Vec<u8>{
    let n = (num & 0xff) as u8;
    vec![n]
}

pub fn encode_i16(num: i32) -> Vec<u8>{
    let n1 = (num & 0xff) as u8;
    let n2 = ((num >> 8) & 0xff) as u8;
    vec![n2, n1]
}

pub fn encode_i32(num: i32) -> Vec<u8>{
    let n1 = (num & 0xff) as u8;
    let n2 = ((num >> 8) & 0xff) as u8;
    let n3 = ((num >> 16) & 0xff) as u8;
    let n4 = ((num >> 24) & 0xff) as u8;
    vec![n4, n3, n2, n1]
}

pub fn encode_f32(num: f32) -> Vec<u8>{
    let bb = num.to_bits();
    let n1 = (bb & 0xff) as u8;
    let n2 = ((bb >> 8) & 0xff) as u8;
    let n3 = ((bb >> 16) & 0xff) as u8;
    let n4 = ((bb >> 24) & 0xff) as u8;
    vec![n4, n3, n2, n1]
}

pub fn decode_i16(buf: Vec<u8>) -> i32 {
    let n1: u16 = (buf[0] as u16) << 8;
    let num = buf[1] as u16 | n1;
    num as i32
}

pub fn decode_i32(buf: Vec<u8>) -> i32 {
    let n1: u32 = (buf[0] as u32) << 8;
    let n2: u32 = (buf[1] as u32) << 16;
    let n3: u32 = (buf[2] as u32) << 24;
    let num = buf[3] as u32 | n3 | n2 | n1;
    num as i32
}

pub fn encode_str(str1: String) -> Vec<u8> {
    let bb = str1.into_bytes();
    bb
}

pub fn str_pstr(str1: String) -> Vec<u8> {
    let mut bb = str1.into_bytes();
    let len = bb.len();
    bb.insert(0, len as u8);
    bb
}