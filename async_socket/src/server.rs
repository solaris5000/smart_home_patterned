use tokio::net::{TcpListener, ToSocketAddrs};

pub struct SocketServer {
    pub tcp: TcpListener,
}

impl SocketServer {
    pub async fn start_server<Addrs>(addr: Addrs) -> Option<SocketServer>
    where
        Addrs: ToSocketAddrs,
    {
        let temp = TcpListener::bind(addr).await;
        match temp {
            Err(e) => {
                println!("Server starting error. Terminating program.\n{e}");
                None
            }
            Ok(ok) => Some(SocketServer { tcp: ok }),
        }
    }
}

impl std::fmt::Debug for SocketServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is a socket")
    }
}
