//описать методы обмена информации между розеткой и клиентом
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

//зашарить реализацию сервера розетки, клиента
pub mod client;
pub mod server;

pub async fn send_command(data: &[u8; 4], stream: &mut TcpStream) -> bool {
    println!(
        "Outcomming command: {:?}",
        String::from_utf8(Vec::from(*data))
    );
    match stream.writable().await {
        Err(e) => {
            println!("send error : {e}");
            false
        }
        Ok(_) => {
            let rst = stream.try_write(data);

            match rst {
                Err(e) => {
                    println!("{e}");
                    false
                }
                Ok(_) => {
                    println!("Command sended");
                    true
                }
            }
        }
    }
}

pub async fn read_command(stream: &mut TcpStream) -> Option<String> {
    let mut buf = [0u8; 4];

    let read_result = stream.read_exact(&mut buf).await;

    match read_result {
        Err(e) => {
            println!("Read command error: {e}");
            None
        }
        Ok(len) => {
            if len == 4 {
                Some(
                    String::from_utf8(Vec::from(buf))
                        .unwrap_or("Encoding error. Use UTF-8.".to_owned()),
                )
            } else {
                Some("CMD LENGTH ERR. Expected 4 bytes len".to_string())
            }
        }
    }
}
