#![cfg_attr(not(test), no_std)]

#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
use panic_halt as _;
use ufmt;

//#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let water_level = pins.a2.into_analog_input(&mut adc);
    let soil_moisture = pins.a1.into_analog_input(&mut adc);
    
    loop {
        arduino_hal::delay_ms(1000);
        let voltage = water_level.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Water level: {}", voltage).unwrap();
        let voltage = soil_moisture.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Soil moisture: {}", voltage).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_check() {
        assert!(true);
    }
}