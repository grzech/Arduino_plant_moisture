#![cfg_attr(not(test), no_std)]

#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use panic_halt as _;
use ufmt;

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    let mut i = 0u16;
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        ufmt::uwriteln!(&mut serial, "Hello for {} time", i).unwrap();
        if i < u16::MAX {
            i += 1;
        } else {
            i = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_check() {
        assert!(true);
    }
}