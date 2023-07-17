use smart_home::smarthome::{HomeBuilder, Room};


#[tokio::main(flavor = "current_thread")]
async fn main() {

    let test_home = HomeBuilder::new().name("test".to_string()).addr("127.0.0.1".to_string()).build().unwrap();

    let test_home_with_rooms = HomeBuilder::new()
    .name("test".to_string())
    .addr("127.0.0.1".to_string())
    .names_of_rooms("Room1, Room2,Room3".to_string())
    .build()
    .unwrap();


    let rooms = vec![Room::new("TestRoom1").unwrap()];

    let test_home_with_roomsvec = HomeBuilder::new()
    .name("test".to_string())
    .addr("127.0.0.1".to_string())
    .udp(111)
    .tcp(111)
    .rooms(rooms)
    .build()
    .unwrap();

    dbg!(test_home);
    dbg!(test_home_with_rooms);
    dbg!(test_home_with_roomsvec);


    println!("Hello, world!");
}