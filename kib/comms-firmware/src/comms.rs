use atsamd_hal as hal;

use hal::sercom::*;

pub struct CommDriver<SCL, SDA, INT> {
    scl: SCL,
    sda: SDA,
    int: INT,
}

impl<SCL, SDA, INT> CommDriver<SCL, SDA, INT> 
where SCL: hal::gpio::Pin,
      SDA: hal::gpio::Pin,
      INT: hal::gpio::Pin,{
    pub fn new(scl: SCL, sda: SDA, int: INT) -> Self {
        Self {
            scl: SCL,
            sda: SDA,
            int: INT,
        }
    }

    pub fn initialize(&mut self) {}
}
