use keyboard_matrix::KeyboardMatrix;
use synth_engine::SynthEngine;

use illuminator::IlluminationEngine;

use comms::BusCommand;
use comms::BusStatus;

use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;

pub fn process_command<LedStrand>(command: &BusCommand, synth_engine: &mut SynthEngine, illumination_engine: &mut IlluminationEngine<LedStrand>) 
where LedStrand: SmartLedsWrite<Error = (), Color = RGB8> {
    match command.register {
        0x10 => {
            if command.data_size == 1 && (1..9).contains(&command.data[0]) {
                synth_engine.set_octave(command.data[0])
            }
        }

        _ => { }
    }
}

pub fn build_response<LedStrand>(register: u8, synth_engine: &SynthEngine, illumination_engine: &IlluminationEngine<LedStrand>) -> Option<([u8; 20], usize)>
where LedStrand: SmartLedsWrite<Error = (), Color = RGB8> {
    
    let mut register_data: [u8; 20] = [0; 20];

    match register {
        0x10 => {
            let octave = synth_engine.state.octave;

            register_data[0] = octave;

            Some((register_data, 1))
        }
        _ => { 
            None
        }
    }
}