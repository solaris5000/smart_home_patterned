// модуль описывающий ошибки

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeError {
    #[error("There are same udp and tcp port")]
    SamePorts,
    #[error("There are already room with the same name : {0}")]
    DuplicatingRoom(String),
    #[error("Room {0} is not exitst")]
    RoomNotExist(String),
    #[error("Some device error in home")]
    InnerDeviceError,
    #[error("Some room error in home: {0}")]
    InnerRoomError(#[from] RoomError),
}


#[derive(Error, Debug)]
pub enum RoomError {
    #[error("Minimal name length requires 1 symbol")]
    MinNameLength,
    #[error("The room has no devices")]
    NoDevices,
}