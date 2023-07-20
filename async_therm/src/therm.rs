#[allow(dead_code)]
#[derive(Debug)]
pub struct Thermometer {
    pub name: String,
    pub room_name: String,
    pub temp: f32,
}

impl std::fmt::Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Thermometer: {} \nLocation: {}",
            self.name, self.room_name
        )
    }
}

impl Thermometer {
    pub fn new(name: &str) -> Self {
        Thermometer {
            name: (name.to_owned()),
            room_name: ("Unknown".to_owned()),
            temp: (0.0),
        }
    }

    pub fn _scan_temp(&mut self) {
        todo!();
    }

    pub fn _get_temp(&mut self) {
        self._scan_temp();
        todo!();
    }
}
