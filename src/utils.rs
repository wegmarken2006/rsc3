use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn print_bytes(name: &str, lst: &Vec<u8>) {
    println!("{}", name);
    let mut ascii: String = "".to_string();
    for elem in lst {
        if *elem >= 32 && *elem <= 126 {
            ascii.push(*elem as char);
            //print!(" {:?}", *elem as char);
        } else {
            if ascii.len() > 0 {
                print!(" {:?}", &ascii);
                ascii = "".to_string();
            }
            print!(" {:x}", *elem);
        }
    }
    println!("");
}

fn read_file() {
    let f = match File::open("C:\\Work\\RsProj\\rsc3\\src\\out.txt") {
        Ok(file) => file,
        Err(err) => panic!("File not found")
    };
    let mut out: Vec<u8> = Vec::new();

    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let l1 = match line {
            Ok(line) => line,
            Err(_) => {
                "".to_string()
            }
        };
        if l1 != "".to_string() {
            let r1 = l1.trim().parse();
            let b1 = match r1 {
                Ok(num) => num,
                Err(_) => -1,
            };

            out.push(b1 as u8);
        }      

        //let b1: u8 = read!("{}", "255".bytes());
    } 
    println!("{}", out.len());
    print_bytes("RES", &out);
}
