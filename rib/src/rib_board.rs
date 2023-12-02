extern crate cortex_m_rt;
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;

pub use hal::ehal;
pub use hal::pac;

#[cfg(feature = "rib_v0")]
hal::bsp_pins! {
    PA02 {
        name: matrix_a
    }
    PA03 {
        name: matrix_b
    }
    PA04 {
        name: matrix_e
    }
    PA05 {
        name: matrix_d
    }
    PA06 {
        name: matrix_c
    }
    PA10 {
        name: enc_b
    }
    PA11 {
        name: enc_a
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
    PA27 {
        name: enc_sw
    }
}

#[cfg(feature = "rib_v1")]
hal::bsp_pins! {
    PA02 {
        name: addr_set
    }
    PA03 {
        name: matrix_a
    }
    PA04 {
        name: matrix_b
    }
    PA05 {
        name: matrix_e
    }
    PA06 {
        name: matrix_d
    }
    PA07 {
        name: matrix_c
    }
    PA10 {
        name: enc_b
    }
    PA11 {
        name: enc_a
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
    PA27 {
        /// RST pin
        name: enc_sw
    }
}