#![cfg_attr(not(test), no_std)]

#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use panic_halt as _;

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_check() {
        assert!(true);
    }
}