#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
}

// асинхронный сервер, который предоставляет доступ к дому для разных клиентов
// дом будет singleton и доступ к нему будет осуществлён через RwLock

// объект дом будет структурированной точкой для запросов к девайсам, так же временным хранилищем данных