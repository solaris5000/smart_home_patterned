use std::io::Read;

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
            "enbl" => "Socket enabled".to_string(),
            "dsbl" => "Socket disabled".to_string(),
            "ebld" => "Current state: Enabled".to_string(),
            "dbld" => "Current state: Disabled".to_string(),
            "E_WC" => "Error: Wrong command".to_string(),
            "IORE" => "Error: some I/O error".to_string(),
            _ => "Something went wrong while reading responce".to_string(),
        }
    }
}
