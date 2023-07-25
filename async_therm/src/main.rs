use std::sync::Arc;
use tokio::net::UdpSocket;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let udp = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
    let server = "127.0.0.1:50002";

    let mut x = 0f64;
    loop {
        let arc_udp = udp.clone();
        x += 1.33;
        let _ = tokio::time::timeout(tokio::time::Duration::from_secs(10), async move {
            let temp = (20.0 + x.sin() * x.tan()) as i32;
            let send_result = arc_udp.send_to(&i32::to_be_bytes(temp), server).await;
            if let Err(e) = send_result {
                println!("Send err : {e}")
            };
            tokio::time::sleep(tokio::time::Duration::from_millis(7600)).await;
        })
        .await;
        if x > 125.0 {
            break;
        };
    }
}
