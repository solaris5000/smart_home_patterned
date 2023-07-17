// модуль описывающий ошибки

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeError {
    #[error("There are same udp and tcp port")]
    SamePorts,
    #[error("There are already room with the same name")]
    DuplicatingRoom,
    #[error("Some device error in home")]
    InnerDeviceError,
    #[error("Some room error in home")]
    InnerRoomError,
}