use std::net;
use std::mem;
use std::time::Duration;
use sc3::{print_bytes};

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

struct Message<'a> {
    name: &'a str,
    l_datum: Vec<Datum>,
}

fn encode_message(message: Message) -> Vec<u8> {
    let mut es = encode_string(&message.name.to_string());
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

static mut PCFG: PortConfig = PortConfig {
    socket: None,
    addr: None,
};

pub fn sc_start() {
    osc_set_port();
    let msg1 = Message{name: "/notify", l_datum: vec![Datum::Int(1)]};
	//b'/notify\x00,i\x00\x00\x00\x00\x00\x01'
	send_message(msg1);
	let msg2 = Message{name: "/g_new", 
    l_datum: vec![Datum::Int(1), Datum::Int(1), Datum::Int(0)]};
	send_message(msg2);
}

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
    socket.set_write_timeout(Some(Duration::new(2, 0))).expect("Send timeout");
    socket.set_read_timeout(Some(Duration::new(2, 0))).expect("Receive timeout");
    unsafe {
        PCFG.socket = Some(socket);
    }
}

fn osc_send(nmsg: Vec<u8>) {
    //SEND
    unsafe {
        let listen_on: net::SocketAddrV4 = PCFG.addr.unwrap();
        let socket = match &PCFG.socket {
            Some(sock) => sock,
            _ => panic!("osc_send socket")
        };

        let result = &socket.send_to(&nmsg, listen_on);
        //drop(socket);
        match result {
            Ok(amt) => println!("Sent {} bytes", amt),
            Err(err) => panic!("Write error: {}", err),
        }
    }

    osc_receive();
}
fn osc_receive() {
    unsafe {
        let listen_on: net::SocketAddrV4 = PCFG.addr.unwrap();
        let socket = match &PCFG.socket {
            Some(sock) => sock,
            _ => panic!("osc_send")
        };
        //RECEIVE
        //let mut buf: [u8; 1] = [0; 1];
        let mut buf: [u8; 512] = mem::uninitialized();
        println!("Reading data");
        let result = socket.recv_from(&mut buf);
        //drop(socket);
        let mut data;
        match result {
            Ok((amt, src)) => {
                //println!("Received data from {}", src);
                data = Vec::from(&buf[0..amt]);
                print_bytes("Received data", &data);
            }
            Err(err) => panic!("Read error: {}", err),
        }
    }
}
