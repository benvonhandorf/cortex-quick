use keyboard_matrix::KeyboardState;
use smart_leds::hsv::RGB8;

pub struct LedState {
    pub leds: [RGB8; 21],
}

impl LedState {
    pub fn new() -> Self {
        Self {
            leds: [RGB8::default(); 21],
            brightness: 0,
        }
    }
}

pub struct Illuminator<LED_STRING> {
    pub led_string: LED_STRING
}

impl Illuminator 
where LED_STRING: SmartLedsWrite<Error = ()>{
    pub fn new(&mut led_string: LED_STRING) -> Self {
        Self {led_string: led_string}
    }

    pub fn update(&mut self, keyboard_state: &KeyboardState) -> LedState {
        let mut led_state = LedState::new();

        for i in 0..21 {
            if keyboard_state.state[i] {
                led_state.leds[i] = RGB8 {
                    r: 255,
                    g: 255,
                    b: 255,
                };
            }
        }

        led_state
    }

    pub fn render(&mut self) {
        let mut led_string = ws2812::Ws2812::new(led_timer, led_data_pin);

        let color_sequence = [
            RGB8 { r:128, g:128,b: 55,},
            RGB8 { r:168, g:125,b: 61,},
            RGB8 { r:203, g:116,b: 64,},
            RGB8 { r:232, g:102,b: 64,},
            RGB8 { r:250, g:84,b: 61,},
            RGB8 { r:255, g:64,b: 55,},
            RGB8 { r:250, g:44,b: 47,},
            RGB8 { r:232, g:26,b: 37,},
            RGB8 { r:203, g:12,b: 27,},
            RGB8 { r:168, g:3,b: 17,},
            RGB8 { r:128, g:0,b: 9,},
            RGB8 { r:88, g:3,b: 3,},
            RGB8 { r:53, g:12,b: 0,},
            RGB8 { r:24, g:26,b: 0,},
            RGB8 { r:6, g:44,b: 3,},
            RGB8 { r:0, g:64,b: 9,},
            RGB8 { r:6, g:84,b: 17,},
            RGB8 { r:24, g:102,b: 27,},
            RGB8 { r:53, g:116,b: 37,},
            RGB8 { r:88, g:125,b: 47,},
        ];
        let mut infinite_color_sequence = color_sequence.iter().cycle();

        led_string.write(led_data.iter().cloned()).unwrap();
    }

}
