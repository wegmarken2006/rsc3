use std::net;

pub fn encode_i8(num: i32) -> Vec<u8> {
    let n = (num & 0xff) as u8;
    vec![n]
}

pub fn encode_i16(num: i32) -> Vec<u8> {
    let n1 = (num & 0xff) as u8;
    let n2 = ((num >> 8) & 0xff) as u8;
    vec![n2, n1]
}

pub fn encode_i32(num: i32) -> Vec<u8> {
    let n1 = (num & 0xff) as u8;
    let n2 = ((num >> 8) & 0xff) as u8;
    let n3 = ((num >> 16) & 0xff) as u8;
    let n4 = ((num >> 24) & 0xff) as u8;
    vec![n4, n3, n2, n1]
}

pub fn encode_f32(num: f32) -> Vec<u8> {
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

pub fn encode_str(str1: &String) -> Vec<u8> {
    let bb = str1.clone().into_bytes();
    bb
}

pub fn str_pstr(str1: &String) -> Vec<u8> {
    let mut bb = str1.clone().into_bytes();
    let len = bb.len();
    bb.insert(0, len as u8);
    bb
}

fn align(n: i32) -> i32 {
    4 - n % 4
}

fn extend_(pad: u8, bts: &Vec<u8>) -> Vec<u8> {
    let n = align(bts.len() as i32);
    let mut out = Vec::new();
    out.extend(bts);
    for ind in 0..n {
        out.push(pad);
    }
    out
}

fn encode_string(str1: &String) -> Vec<u8> {
    extend_(0 as u8, &encode_str(str1))
}

fn encode_blob(bts: &Vec<u8>) -> Vec<u8> {
    let b1 = encode_i32(bts.len() as i32);
    let mut out = Vec::new();
    out.extend(b1);
    out.extend(extend_(0 as u8, bts));
    out
}

#[derive(Clone, PartialEq, Debug)]
enum Datum {
    Int(i32),
    Float(f32),
    Str(String),
    Blob(Vec<u8>),
}

fn encode_datum(datum: &Datum) -> Vec<u8> {
    match datum {
        Datum::Int(int) => encode_i32(*int),
        Datum::Float(float) => encode_f32(*float),
        Datum::Str(strng) => encode_string(strng),
        Datum::Blob(blob) => encode_blob(blob),
    }
}

fn tag(datum: &Datum) -> char {
    match datum {
        Datum::Int(int) => 'i',
        Datum::Float(float) => 'f',
        Datum::Str(strng) => 's',
        Datum::Blob(blob) => 'b',
    }
}

fn descriptor(id: Vec<Datum>) -> String {
    let mut outs: String = ",".to_string();
    for dt in id {
        outs.push(tag(&dt));
    }
    outs
}

struct Message {
    name: String,
    l_datum: Vec<Datum>,
}

fn encode_message(message: Message) -> Vec<u8> {
    let mut es = encode_string(&message.name);
    let ds1 = encode_string(&descriptor(message.l_datum.clone()));
    let mut ds2 = Vec::new();
    for elem in message.l_datum {
        ds2.extend(encode_datum(&elem))
    }
    es.extend(ds1);
    es.extend(ds2);
    es
}

fn send_message(message: Message) {
    let bmsg = encode_message(message);
    osc_send(bmsg);
}


struct PortConfig {
    socket: Option<net::UdpSocket>,
    addr: Option<net::SocketAddrV4>,
}

static mut PCFG: PortConfig  = PortConfig {
    socket: None,
    addr: None
};

fn osc_set_port() {
    let listen_on = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 57110);
    unsafe {
        PCFG.addr = Some(listen_on);
    }
    
    let attempt = net::UdpSocket::bind(listen_on);
    let socket = match attempt {
        Ok(sock) => sock,
        Err(err) => panic!("Could not bind: {}", err),
    };
    unsafe {
        PCFG.socket = Some(socket);
    }    
}

fn osc_send<'a>(nmsg: Vec<u8>) {
    //SEND
    let listen_on:  net::SocketAddrV4;
    let socket: &'a net::UdpSocket; 
    unsafe {
        listen_on = PCFG.addr.unwrap();
        socket = &PCFG.socket.unwrap();
    }
    
    let result = socket.send_to(&nmsg, listen_on);
    //drop(socket);
    match result {
        Ok(amt) => println!("Sent {} bytes", amt),
        Err(err) => panic!("Write error: {}", err),
    }

    //RECEIVE
    let mut buf: [u8; 1] = [0; 1];
    println!("Reading data");
    let result = socket.recv_from(&mut buf);
    drop(socket);
    let mut data;
    match result {
        Ok((amt, src)) => {
            println!("Received data from {}", src);
            data = Vec::from(&buf[0..amt]);
        }
        Err(err) => panic!("Read error: {}", err),
    }
}
