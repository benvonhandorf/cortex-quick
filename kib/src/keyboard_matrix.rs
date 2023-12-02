use atsamd_hal::ehal::digital::v2::{InputPin, OutputPin};

pub struct KeyboardMatrix<ROW, COL>
{
    row_c: ROW,
    col_p: COL,
    delay: u32,
}

impl<ROW, COL> KeyboardMatrix<ROW, COL>
where
    ROW: OutputPin,
    COL: InputPin,
{
    pub fn new(row_c: ROW, col_p: COL) -> Self {
        Self { row_c, col_p, delay: 0 }
    }

    pub fn scan(&mut self) -> bool {
        self.row_c.set_low().ok();
        self.row_c.set_high().ok();
        let mut counter: u32 = 0;
        let mut pressed = self.col_p.is_low().unwrap_or(false);

        while !pressed {
            counter += 1;
            if counter > 1000 {
                break;
            }
            pressed = self.col_p.is_low().unwrap_or(false);
        }

        self.delay = counter;
        
        pressed
    }
}
