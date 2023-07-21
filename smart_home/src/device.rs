// модуль описывает взаимодействие с девайсами

use std::{error::Error, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, UdpSocket},
    sync::RwLock,
    time::timeout,
};

use crate::errors::ConnectionError;

#[derive(Debug, Default)]
pub struct Device {
    name: String,
    ip: String,
    device: InnerDevice,
}

#[derive(Debug, Default)]
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

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::SmartSocket
    }
}

/// Функция для установления соединения с устройствами.
/// В случае TCP устройства - происходит обмен сообщениями, с целью установления корректности адресата.
/// В случае UDP устройства - происходит получение датаграмм на указанный сокет, их расшифровка и в случае корректного паттерна
/// возвращаем успех в установлении соединения.
pub async fn handshake(
    incoming_udp_socket: Option<&UdpSocket>,
    device: &Device,
) -> DeviceHandshakeResult {
    match device.device_type {
        DeviceType::SmartSocket => tcp_handshake(device).await,
        DeviceType::SmartThermometer => udp_try_recive(incoming_udp_socket.unwrap(), device).await,
    }
}

//НЕОБХОДИМА РЕАЛИЗАЦИЯ ХЕНДШЕЙКА ДЛЯ УСТРОЙСТВА
async fn tcp_handshake(device: &Device) -> DeviceHandshakeResult {
    if device != InnerDevice::SmartSocket {
        return ConnectionError::WrongDevice("TCP".to_string());
    }
    let mut stream = TcpStream::connect(device.ip).await;

    if stream.is_err() {
        println!(
            "[ERROR] some error happend due handshake to '{}'; Error: {}",
            device.ip,
            stream.unwrap_err()
        );
        Err(ConnectionError::BadHandshakeResult(device.ip));
    } else {
        let stream = stream.unwrap();
        let buf: &[u8; 4] = "HDSH".as_bytes();

        match timeout(Duration::from_secs(3), stream.write_all(buf)).await {
            Ok(_) => {}
            Err(_) => {
                return Err(ConnectionError::ConnectionTimeout(device.ip));
            }
        }

        buf.clear();

        match timeout(Duration::from_secs(3), stream.read_exact(buf)).await {
            Ok(_) => match buf {
                "HSDS" => Ok(stream),
                _ => Err(ConnectionError::BadHandshakeResult(device.ip)),
            },
            Err(_) => Err(ConnectionError::ConnectionTimeout(device.ip)),
        }
    }
}

// НЕ РЕАЛИЗОВАННО НА СТОРОНЕ ТЕРМОМЕТРА
// сделать протокол : старший байт - буква T, младшие 3 байта - температура

// нужно дописать реализацию с Arc<Rwlock домом, чтобы при приходе датаграммы, проверялись комнаты и находился нужный термометр, данные о котором будут апдейтиться
async fn listen_udp(incoming_socket: &UdpSocket, home: Arc<RwLock<Home>>) -> std::io::Result<()> {
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    loop {
        let homearc = home.clone();
        buf = [0, 0, 0, 0];

        let (len, addr) = incoming_socket.recv_buf_from(&mut buf).await?;

        if size == 4_usize {
            if buf[0] == 'T' {
                buf[0] == 0_u8;
                let temp = f32::from_le_bytes(&buf);
                dbg!(temp);
                {
                   let guard = home.write();
                   
                   for room in guard.rooms {
                        for device in room.device {
                            if device.ip == addr.to_string() {
                                device.0 = Some(temp);
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
    use crate::smarthome::HomeBuilder;

    #[test]
    fn it_works() {
        let stream = tokio::net::TcpStream::connect("123.123.123.123:56566").await;
        if let a = std::io::Error == stream {
            println!("Succsess");
        }
    }

    #[test]
    fn tst() {
        1.5.to_be_bytes();
    }
}
