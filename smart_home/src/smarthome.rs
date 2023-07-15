/// модуль описывает взаимодействие дома с комнатами, с сервером

use crate::{device::Device, errors::HomeError};

#[derive(Debug)]
pub struct Home {
    name : String,
    addr : String,
    rooms : Vec<Room>,
    udp_port : u16,
    tcp_port : u16,
}

#[derive(Debug, Default)]
pub  struct HomeBuilder {
    name : String,
    addr : String,
    rooms : Vec<Room>,
    udp_port : u16,
    tcp_port : u16,
}

#[derive(Debug, Default)]
pub struct Room {
    name : String,
    device : Vec<Device>,
}


impl HomeBuilder {

    pub fn new() -> HomeBuilder {
        HomeBuilder { tcp_port : 50001 , udp_port : 50002, ..Default::default() }
    }

    pub fn name(mut self, name : String ) -> HomeBuilder {
        self.name = name;
        self
    }

    pub fn addr(mut self, name : String ) -> HomeBuilder {
        self.name = name;
        self
    }

    pub fn rooms(mut self, rooms : Vec<Room> ) -> HomeBuilder {
        self.rooms = rooms;
        self
    }

    pub fn udp(mut self, udp : u16 ) -> HomeBuilder{
        self.udp_port = udp;
        self
    }

    pub fn tcp(mut self, tcp : u16 ) -> HomeBuilder{
        self.tcp_port = tcp;
        self
    }
    

    
    pub fn names_of_rooms(mut self, rooms_string : String ) -> HomeBuilder {
        // Принимает в качестве комнат список комнат через запятую

        let room_names : Vec<&str> = rooms_string.split(',').collect();

        for room_name in room_names {
            self.rooms.push(Room::new(room_name.trim()));
        }

        self
    }


    pub fn build(self) -> Result<Home, HomeError> {

        if self.udp_port == self.tcp_port {
            return Err(HomeError::SamePorts);
        }
        Ok(Home { name : self.name, addr : self.addr, rooms : self.rooms, tcp_port : self.tcp_port, udp_port : self.udp_port})
    }
}

impl  Room {
    pub fn new(name : &str) -> Room {
        Room { name: name.to_string() , ..Default::default()}
    }
}