pub use atsamd_hal as hal;

pub use hal::ehal;

pub struct LedMatrix {
    pin_a : Pin<I, PushPullOutput>,
    pin_b : Pin<I, PushPullOutput>,
    pin_c : Pin<I, PushPullOutput>,
    pin_d : Pin<I, PushPullOutput>,
    pin_e : Pin<I, PushPullOutput>,
}

enum MatrixPin {
    PinA,
    PinB,
    PinC,
    PinD,
    PinE,
}

const led_pin_drives: [(MatrixPin, MatrixPin)] = [
    (MatrixPin::PinA, MatrixPin::PinB),
    (MatrixPin::PinA, MatrixPin::PinC),
    (MatrixPin::PinA, MatrixPin::PinD),
    (MatrixPin::PinA, MatrixPin::PinE),
    (MatrixPin::PinB, MatrixPin::PinA),
    (MatrixPin::PinB, MatrixPin::PinC),
    (MatrixPin::PinB, MatrixPin::PinD),
    (MatrixPin::PinB, MatrixPin::PinE),
    (MatrixPin::PinC, MatrixPin::PinA),
    (MatrixPin::PinC, MatrixPin::PinB),
    (MatrixPin::PinC, MatrixPin::PinD),
    (MatrixPin::PinC, MatrixPin::PinE),
    (MatrixPin::PinD, MatrixPin::PinA),
    (MatrixPin::PinD, MatrixPin::PinB),
    (MatrixPin::PinD, MatrixPin::PinC),
    (MatrixPin::PinD, MatrixPin::PinD),
    (MatrixPin::PinE, MatrixPin::PinA),
    (MatrixPin::PinE, MatrixPin::PinB),
    (MatrixPin::PinE, MatrixPin::PinC),
    (MatrixPin::PinE, MatrixPin::PinD),
];