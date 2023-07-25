use std::{io::*, net::TcpStream};

pub fn read_responce<Reader: Read>(mut reader: Reader) -> String {
    let mut buf = [0; 4];
    let responce: (bool, String) = match reader.read_exact(&mut buf) {
        Ok(_) => {
            let vbuf = buf.to_vec();
            let tmp = String::from_utf8(vbuf).unwrap_or("Encoding error. Use UTF-8.".to_owned());
            if tmp != "F32D" {
                (false, tmp)
            } else {
                buf = [0; 4];
                match reader.read_exact(&mut buf) {
                    Ok(_) => (true, f32::from_be_bytes(buf).to_string()),
                    Err(e) => {
                        println!("{e}");
                        (false, "IOER".to_owned())
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}");
            (false, "IOER".to_owned())
        }
    };

    if responce.0 {
        format!("Current power is: {}", responce.1)
    } else {
        match &responce.1[..] {
            "enbl" => // прописать комманды для дома
        }
    }
}

fn main() {
    let mut inp = String::new();
    println!("Welcome to SmartSocketTM(c) CLI.\nList of aviable commands:
    \nenbl - Enable socket\ndsbl - disable socket\npowr - get current power of socket\nstat - get current state of socket
    \nstop - close this CLI\n");
    let mut client = TcpStream::connect("127.0.0.1:10010").unwrap();
    loop {
        inp.clear();
        let _ = std::io::stdin().read_line(&mut inp);
        let input = inp.trim();
        if inp == *"stop\r\n" {
            break;
        }
        let _ = client.write_all(input.as_bytes());
        println!("{}", read_responce(&client));
    }
}
