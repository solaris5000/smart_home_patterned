// модуль описывает взаимодействие с девайсами

use tokio::net::TcpStream;


#[derive(Debug, Default)]
pub struct Device {
    name : String,
    ip : String,
    device_type : DeviceType
}


#[derive(Debug)]
enum DeviceType {
    SmartSocket,
    SmartThermometer    
}

pub struct DeviceHandshakeResult {
    status : bool,
    tcp_stream : Option<TcpStream>
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::SmartSocket
    }
}




pub async fn handshake(device : &Device) -> DeviceHandshakeResult {
    match device.device_type {
        DeviceType::SmartSocket => { tcp_handshake(device).await },
        DeviceType::SmartThermometer => { udp_try_recive(device).await },
    }
}

async fn tcp_handshake(device : &Device) -> DeviceHandshakeResult {
    // отправляем в розетку handshake, ждём результат, в случае если удаётся, отдаём TCPListener

    DeviceHandshakeResult{status : false, tcp_stream : None }
}

async fn udp_try_recive(device : &Device) -> DeviceHandshakeResult {
        // попытаться получить от термометра данные в нужном формате, если данные получены правильно, 
        // то продолжаем работать с этой розеткой, иначе отменяем работу сней
        DeviceHandshakeResult{status : false, tcp_stream : None }
}