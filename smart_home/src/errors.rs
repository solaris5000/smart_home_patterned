//Модуль, в котором описываются ошибки

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmartError {
    #[error(transparent)]
    SmartRoomErr(#[from] RoomError),
    #[error(transparent)]
    SmartDeviceErr(#[from] DeviceError),
}

#[derive(Error, Debug)]
pub enum RoomError {
    #[error("There is already room named {0}")]
    ExistingRoomErr(String),
    #[error("The home has no room named {0}")]
    RoomIsNotExist(String),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Device '{0}' is not attached to any room")]
    DeviceNoAttached(String),
    #[error("The room already has device named '{0}'")]
    ExistingDeviceErr(String),
}