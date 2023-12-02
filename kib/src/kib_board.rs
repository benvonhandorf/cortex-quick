extern crate cortex_m_rt;
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::ehal;
pub use hal::pac;

hal::bsp_pins! {
    PA02 {
        name: addr_set
    }
    PA03 {
        name: row_a
    }
    PA04 {
        name: row_b
    }
    PA05 {
        name: row_c
    }
    PA06 {
        name: row_e
    }
    PA07 {
        name: row_d
    }
    PA10 {
        name: col_p
    }
    PA11 {
        name: col_o
    }
    PA14 {
        name: sda
        aliases: {
            AlternateC: Sda
        }
    }
    PA15 {
        name: scl
        aliases: {
            AlternateC: Scl
        }
    }
    PA16 {
        name: int
    }
    PA17 {
        name: col_q
    }
    PA22 {
        name: com_n
    }
    PA23 {
        name: com_m
    }
    PA27 {
        name: led_data
    }
}
