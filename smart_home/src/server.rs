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
        "placeholder" => {}
        _ => {
            sdtp::send_command(b"E_WC", stream).await;
        }
    }
    ConnectionState::CsConnected
}