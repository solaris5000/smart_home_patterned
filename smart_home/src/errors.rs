// модуль описывающий ошибки

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeError {
    #[error("There are same udp and tcp port")]
    SamePorts,
}