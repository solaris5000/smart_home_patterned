use std::ops::Index;

/// модуль описывает взаимодействие дома с комнатами, с сервером
use crate::{
    device::{Device, DeviceType},
    errors::{HomeError, RoomError},
};

#[derive(Debug)]
pub struct Home {
    name: String,
    addr: String,
    rooms: Vec<Room>,
    udp_port: u16,
    tcp_port: u16,
}

#[derive(Debug, Default)]
pub struct HomeBuilder {
    name: String,
    addr: String,
    rooms: Vec<Room>,
    udp_port: u16,
    tcp_port: u16,
}

#[derive(Debug, Default)]
pub struct Room {
    name: String,
    device: Vec<Device>,
}

pub struct Report(String);

impl HomeBuilder {
    pub fn new() -> HomeBuilder {
        HomeBuilder {
            addr: "127.0.0.1".to_string(),
            tcp_port: 50001,
            udp_port: 50002,
            ..Default::default()
        }
    }

    pub fn name(mut self, name: String) -> HomeBuilder {
        self.name = name;
        self
    }

    pub fn addr(mut self, name: String) -> HomeBuilder {
        self.name = name;
        self
    }

    pub fn rooms(mut self, rooms: Vec<Room>) -> HomeBuilder {
        self.rooms = rooms;
        self
    }

    pub fn udp(mut self, udp: u16) -> HomeBuilder {
        self.udp_port = udp;
        self
    }

    pub fn tcp(mut self, tcp: u16) -> HomeBuilder {
        self.tcp_port = tcp;
        self
    }

    /// Позволяет при создании дома создать пустые комнаты по названию, указанными через запятую.
    /// # Examples
    /// ```
    /// let a = HomeBuilder::new()
    /// .name("test".to_string())
    /// .addr("127.0.0.1".to_string())
    /// .names_of_rooms("Room1, Room2,Room3".to_string())
    /// .build()
    /// .unwrap();
    /// ```
    pub fn names_of_rooms(mut self, rooms_string: String) -> HomeBuilder {
        let room_names: Vec<&str> = rooms_string.split(',').collect();

        for room_name in room_names {
            self.rooms.push(Room::new(room_name.trim()).unwrap());
        }

        self
    }

    pub fn build(self) -> Result<Home, HomeError> {
        if self.udp_port == self.tcp_port {
            return Err(HomeError::SamePorts);
        }
        Ok(Home {
            name: self.name,
            addr: self.addr,
            rooms: self.rooms,
            tcp_port: self.tcp_port,
            udp_port: self.udp_port,
        })
    }
}

impl Home {
    // добавление комнаты
    pub fn add_room(&mut self, room: Room) -> Result<(), HomeError> {
        for inner_room in &self.rooms {
            if inner_room.name == room.name {
                return Err(HomeError::DuplicatingRoom(room.name.to_string()));
            }
        }
        Ok(self.rooms.push(room))
    }

    /// Создание комнаты по указанному имени.
    pub fn create_room(&mut self, room_name: &str) -> Result<(), HomeError> {
        for inner_room in &self.rooms {
            if inner_room.name == room_name {
                return Err(HomeError::DuplicatingRoom(room_name.to_string()));
            }
        }
        Ok(self.rooms.push(Room::new(room_name).unwrap()))
    }

    ///Удаление комнаты из дома. Если комната не существует или в процессе удаления возникла ошибка, возвращает ошибку.
    pub fn remove_room(&mut self, room_name: &str) -> Result<(), HomeError> {
        //let mut operating_room = &mut Room::placeholder();
        let mut index = 0usize;
        for inner_room in &mut self.rooms {
            if inner_room.name == room_name {
                match inner_room.clear_devices() {
                    Ok(_) => {
                        self.rooms.remove(index);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("{:?}", &e);
                        match e {
                            RoomError::NoDevices => {
                                self.rooms.remove(index);
                                return Ok(());
                            }
                            _ => {
                                return Err(e.into());
                            }
                        }
                    }
                }
            }

            index += 1;
        }

        Err(HomeError::RoomNotExist(room_name.to_string()))
    }

    ///Создание отчёта по всем комнатам в доме.
    pub fn home_report(&self) -> Result<String, HomeError> {
        let mut report = "Home report:".to_string();

        let level = "\n\t";

        for room in &self.rooms {
            report = report + level + &room.room_report().unwrap();
        }

        Ok(report)
    }

    ///Отображение сетевой информации о доме.
    pub fn show_config(&self) {
        println!("IP address: {}", self.addr);
        println!("TCP port: {}", self.tcp_port);
        println!("UDP port: {}", self.udp_port);
    }
}

impl Room {
    pub fn new(name: &str) -> Result<Room, RoomError> {
        if name.trim() == "" {
            return Err(RoomError::MinNameLength);
        }
        Ok(Room {
            name: name.to_string(),
            ..Default::default()
        })
    }

    ///Затычка, используемая в качестве нулевой комнаты с невозможным пустым именем.
    fn placeholder() -> Room {
        Room {
            name: "".to_string(),
            ..Default::default()
        }
    }

    ///Создание отчёта комнаты.
    pub fn room_report(&self) -> Result<String, HomeError> {
        let mut report = "Room name: ".to_string() + &self.name;
        let level = "\n\t\t";
        if self.device.len() == 0 {
            report = report + level + "There is no devices in this room";
        } else {
            report = report + level + "Device report";
        }
        Ok(report)
    }

    /// Удаление всей информации об устройствах в комнате. Очистка вектора устройств.
    /// В случае если комната пуста, возвращает ошибку.
    fn clear_devices(&mut self) -> Result<(), RoomError> {
        if self.device.len() == 0 {
            return Err(RoomError::NoDevices);
        }

        self.device.clear();
        Ok(())
    }

    pub fn add_device(&mut self, device: Device) {
        self.device.push(device)
    }

    pub fn create_device(&mut self, name: String, ip: String, device_type: DeviceType) {
        self.device.push(Device::new(name, ip, device_type))
    }
}

#[cfg(test)]
mod tests {
    use crate::smarthome::HomeBuilder;

    #[test]
    fn it_works() {
        let mut a = HomeBuilder::new().build().unwrap();
        a.create_room("Test1").unwrap();
        a.create_room("Test Room 2").unwrap();
        println!("{}", a.home_report().unwrap());
    }

    #[test]
    fn ips() {
        let a = HomeBuilder::new().build().unwrap();
        a.show_config();
    }
}
