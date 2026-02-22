use arduino_hal::port::{
    mode::{Input, PullUp},
    Pin, PinOps,
};

pub fn read_dpad_input(
    right: &Pin<Input<PullUp>, impl PinOps>,
    up: &Pin<Input<PullUp>, impl PinOps>,
    left: &Pin<Input<PullUp>, impl PinOps>,
    down: &Pin<Input<PullUp>, impl PinOps>,
) -> [i8; 2] {
    [
        right.is_low() as i8 - left.is_low() as i8,
        down.is_low() as i8 - up.is_low() as i8,
    ]
}
