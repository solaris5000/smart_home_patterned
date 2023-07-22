// модуль описывает взаимодействие с девайсами

use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, UdpSocket},
    sync::RwLock,
    time::timeout,
    time::Duration
};

use crate::{errors::ConnectionError, smarthome::Home};

#[derive(Debug)]
pub struct Device {
    name: String,
    ip: String,
    pub device: InnerDevice,
}

#[derive(Debug)]
pub enum InnerDevice {
    SmartSocket,
    SmartThermometer(Option<f32>),
}

pub struct DeviceHandshakeResult {
    result: Result<TcpStream, ConnectionError>,
}

impl Device {
    pub fn new(name: String, ip: String, device: InnerDevice) -> Device {
        Device {
            name: name,
            ip: ip,
            device: device,
        }
    }
}

/// Функция для установления соединения с устройствами.
/// В случае TCP устройства - происходит обмен сообщениями, с целью установления корректности адресата.
/// В случае UDP устройства - происходит получение датаграмм на указанный сокет, их расшифровка и в случае корректного паттерна
/// возвращаем успех в установлении соединения.
/// 
/*
pub async fn handshake(
    incoming_udp_socket: Option<&UdpSocket>,
    device: &Device,
) -> DeviceHandshakeResult {
    match device.device {
        InnerDevice::SmartSocket => tcp_handshake(device).await,
        InnerDevice::SmartThermometer(t) => udp_try_recive(incoming_udp_socket.unwrap(), device).await,
    }
}
*/

//НЕОБХОДИМА РЕАЛИЗАЦИЯ ХЕНДШЕЙКА ДЛЯ УСТРОЙСТВА
async fn tcp_handshake(device: &Device) -> DeviceHandshakeResult {
    match device.device {
    _ => {
        return DeviceHandshakeResult {result : Err(ConnectionError::WrongDevice("TCP".to_string()))}
    },
    InnerDevice::SmartSocket => {
    let stream = TcpStream::connect(device.ip.to_string()).await;

    if stream.is_err() {
        println!(
            "[ERROR] some error happend due handshake to '{}'; Error: {}",
            device.ip,
            stream.unwrap_err()
        );
        DeviceHandshakeResult {result : Err(ConnectionError::BadHandshakeResult(device.ip.to_string()))}
    } else {
        let mut stream = stream.unwrap();
        let mut buf = *b"HSDS";

        match timeout(Duration::from_secs(3), stream.write_all(&buf)).await {
            Ok(_) => {}
            Err(_) => {
                return DeviceHandshakeResult { result : Err(ConnectionError::ConnectionTimeout(device.ip.to_string()))};
            }
        }

        buf = [0; 4];
        let timeout = timeout(
        Duration::from_secs(3),
        stream.read_exact(&mut buf)).await;

        match timeout
        {
            Ok(_) => match &buf {
                b"HSDS" => DeviceHandshakeResult { result : Ok(stream)},
                _ => DeviceHandshakeResult { result : Err(ConnectionError::BadHandshakeResult(device.ip.to_string()))},
            },
            Err(_) => DeviceHandshakeResult { result : Err(ConnectionError::ConnectionTimeout(device.ip.to_string()))},
        }
    }
    }
}
}

// НЕ РЕАЛИЗОВАННО НА СТОРОНЕ ТЕРМОМЕТРА
// сделать протокол : старший байт - буква T, младшие 3 байта - температура

// нужно дописать реализацию с Arc<Rwlock домом, чтобы при приходе датаграммы, проверялись комнаты и находился нужный термометр, данные о котором будут апдейтиться
async fn listen_udp(incoming_socket: &UdpSocket, home: Arc<RwLock<Home>>) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(4);
    let mut temp_buf = [0u8; 4];
    loop {
        let homearc = home.clone();
        buf.clear();

        let (len, addr) = incoming_socket.recv_buf_from(&mut buf).await?;

        if len == 4_usize {
            if buf[0] == b'T' {
                temp_buf[1] = buf[1];
                temp_buf[2] = buf[2];
                temp_buf[3] = buf[3];
                let temp = f32::from_le_bytes(temp_buf);
                dbg!(temp);
                {
                   let guard = homearc.write();
                   
                   for room in &mut guard.await.rooms {
                        for mut device in &mut room.device {
                            if device.ip == addr.to_string() {
                                device.device = InnerDevice::SmartThermometer(Some(temp));
                                println!("[INFO] Updated data about {}", device.ip);
                            }
                        }
                   }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
   // use crate::smarthome::HomeBuilder;
/* 
    #[test]
    fn it_works() {
        let stream = tokio::net::TcpStream::connect("123.123.123.123:56566").await;
        if let a = std::io::Error == stream {
            println!("Succsess");
        }
    }*/
}
