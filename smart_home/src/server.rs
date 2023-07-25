//переписать функцию под работу с домом, получать комманды от клиента и работать с домом, после отправлять клиенту ответы

// функиця для поддержания коннекта с клиентом, тоже переписать надо её

pub enum ConnectionState {
    CsConnected,
    CsDisconnected,
}

pub async fn process_connection(connection: TcpStream, guard: Arc<RwLock<Home>>) {
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

async fn scan_command(guard: Arc<RwLock<Home>>, stream: &mut TcpStream) -> ConnectionState {

    // сделать так, что общение происходит посредством 16 байтовых пакетов?
    let home = guard.as_ref();

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