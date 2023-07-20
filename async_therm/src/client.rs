use tokio::net::{ToSocketAddrs, UdpSocket};

pub enum ClientCommand {
    GetTemp,
}

pub struct Client {
    pub udp: UdpSocket,
}

impl Client {
    pub async fn send_command<ADR>(
        &self,
        cmd: ClientCommand,
        target: ADR,
    ) -> Result<usize, std::io::Error>
    where
        ADR: ToSocketAddrs,
    {
        match cmd {
            ClientCommand::GetTemp => self.udp.send_to(b"TEMP", target).await,
        }
    }
}
