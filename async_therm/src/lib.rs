//описать методы обмена информации между розеткой и клиентом
use std::{
    io::{Read, Write},
    vec,
};

//зашарить реализацию сервера розетки, клиента
pub mod client;
pub mod server;
pub mod therm;

pub fn send_command<Writer: Write>(data: [u8; 4], mut writer: Writer) -> bool {
    match writer.write_all(&data) {
        Err(e) => {
            println!("{e}");
            return false;
        }
        Ok(_) => {
            println!("Command sended")
        }
    }
    true
}

pub fn read_command<Reader: Read>(mut reader: Reader) -> String {
    let mut buf: Vec<u8> = vec![0; 4];
    if buf.len() != 4 {
        return "CMD LENGTH ERR. Expected 4 bytes len".to_owned();
    }
    match reader.read_exact(&mut buf) {
        Ok(_) => String::from_utf8(buf).unwrap_or("Encoding error. Use UTF-8.".to_owned()),
        Err(e) => {
            println!("{e}");
            "IOER".to_owned()
        }
    }
}
