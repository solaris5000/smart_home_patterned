use sdtp::server::SocketServer;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Socket {
    pub name: String,
    pub room_name: String,
    pub voltage: f32,
    pub amperage: f32,
    pub power: f32,
    pub enabled: bool,
    //pub address: String,
}

pub enum ConnectionState {
    CsConnected,
    CsDisconnected,
}

pub struct TcpSocket {
    pub tcp: SocketServer,
}

#[allow(clippy::new_without_default)]
impl TcpSocket {
    pub async fn new() -> TcpSocket {
        let temp = SocketServer::start_server("127.0.0.1:0").await;

        match temp {
            None => {
                panic!("There is no free port for this socket server.")
            }
            Some(ok) => TcpSocket { tcp: ok },
        }
    }

    pub async fn accept(
        &self,
    ) -> Result<(tokio::net::TcpStream, std::net::SocketAddr), std::io::Error> {
        self.tcp.tcp.accept().await
    }

    pub async fn process_connection(connection: TcpStream, guard: Arc<RwLock<Socket>>) {
        let mut stream = connection;
        loop {
            let loopguard = guard.clone();
            match Self::scan_command(loopguard, &mut stream).await {
                ConnectionState::CsConnected => {}
                ConnectionState::CsDisconnected => {
                    break;
                }
            }
        }
    }

    async fn scan_command(guard: Arc<RwLock<Socket>>, stream: &mut TcpStream) -> ConnectionState {
        let socket = guard.as_ref();

        let buf = sdtp::read_command(stream).await;
        match buf {
            Some(_) => {}
            None => {
                return ConnectionState::CsDisconnected;
            }
        }
        let buf = &buf.unwrap();
        println!("CMD: {}", &buf);

        match &buf[..] {
            "powr" => {
                let socket = socket.read().await;
                sdtp::send_command(b"F32D", stream).await;
                if socket.enabled {
                    sdtp::send_command(&socket.power.to_be_bytes(), stream).await;
                } else {
                    sdtp::send_command(&0f32.to_be_bytes(), stream).await;
                }
            }
            "stat" => {
                let socket = socket.read().await;
                sdtp::send_command(if socket.enabled { b"ebld" } else { b"dbld" }, stream).await;
            }
            "enbl" => {
                let mut socket = socket.write().await;
                socket.enabled = true;
                sdtp::send_command(b"enbl", stream).await;
            }
            "dsbl" => {
                let mut socket = socket.write().await;
                socket.enabled = false;
                sdtp::send_command(b"dsbl", stream).await;
            }
            "HSDS" => {
                sdtp::send_command(b"SDSH", stream).await;
            }
            "placeholder" => {}
            _ => {
                sdtp::send_command(b"E_WC", stream).await;
            }
        }
        ConnectionState::CsConnected
    }
}

impl std::fmt::Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket: {} \nLocation: {}\nEnabled: {}\nAddress: {}",
            self.name,
            self.room_name,
            if self.enabled {
                format!("Yes\nPower: {}", self.power)
            } else {
                "No".to_owned()
            },
            self.address
        )
    }
}

impl Socket {
    pub fn new(name: &str) -> Self {
        Socket {
            name: (name.to_owned()),
            room_name: ("Unknown".to_owned()),
            voltage: (0.0),
            amperage: (0.0),
            power: (12.5),
            enabled: (false),
           // address: "127.0.0.1:0".to_owned(),
        }
    }
    pub fn _init(&mut self) {
        todo!();
    }

    pub fn on(&mut self) {
        self.enabled = true;
    }

    pub fn off(&mut self) {
        self.enabled = false;
    }

    pub fn _scan_power(&mut self) {
        todo!();
    }

    pub fn _scan_amperage(&mut self, _curr_a: f32) {
        todo!();
    }

    pub fn _scan_voltage(&mut self, _curr_v: f32) {
        todo!();
    }

    pub fn _get_power(&self) {
        println!("Current power is {} W", self.power);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = TcpSocket::new().await;

    let a = Socket::new("MySocket");
    let arcs = Arc::new(RwLock::new(a));
    println!("{}", arcs.read().await);

    loop {
        let connection = server.accept().await;

        match connection {
            Ok(connection_result) => {
                let socket_arc = arcs.clone();
                println!("Connected: {}", &connection_result.1);
                let spawnpeer = connection_result.1.clone();
                tokio::spawn(async move {
                    TcpSocket::process_connection(connection_result.0, socket_arc).await;
                    println!("Disconnecred : {}", spawnpeer);
                });
            }
            Err(e) => {
                println!("Connection is not established, Error : {e}");
            }
        }
    }
}
