#![cfg_attr(not(test), no_std)]

#![cfg_attr(not(test), no_main)]

use ah::Peripherals;
#[cfg(not(test))]
use panic_halt as _;
use ufmt;
use arduino_hal as ah;
use ah::port::{Pin, mode::{Output, Input, Floating, Analog}};
use ah::hal::port::PB4 as LED1;
use ah::hal::port::PB3 as LED2;
use ah::hal::port::PB2 as LED3;
use ah::hal::port::PC1 as ADC1;
use ah::hal::port::PC2 as ADC2;

struct WaterLevel {
    red : Pin<Output, LED1>,
    yellow : Pin<Output, LED2>,
    green : Pin<Output, LED3>,
    level : Pin<Analog, ADC2>,
}

enum SoilState {
    DRY = 600,
    MIDDLE,
    WET = 400,
}

fn get_soil_state(sensor: &Pin<Analog, ADC1>, adc: &mut ah::Adc) -> SoilState {
    let voltage = sensor.analog_read(adc);
    if voltage >= SoilState::DRY as u16 {
        return SoilState::DRY;
    } else if voltage <= SoilState::WET as u16{
        return SoilState::WET
    }
    return SoilState::MIDDLE;
}

fn water_the_plant() {

}

impl WaterLevel {
    const THRESHOLD_HIGH : u16 = 550;
    const THRESHOLD_MEDIUM : u16 = WaterLevel::THRESHOLD_HIGH-1;
    const THRESHOLD_LOW : u16 = 400;
    fn new(red : Pin<Input<Floating>, LED1>, yellow : Pin<Input<Floating>, LED2>,
           green : Pin<Input<Floating>, LED3>, analog_in: Pin<Analog, ADC2>) -> WaterLevel {
            WaterLevel{red: red.into_output(), yellow: yellow.into_output(), 
                green: green.into_output(), level: analog_in}
    }

    fn red_active(&mut self) {
        self.red.set_high();
        self.yellow.set_low();
        self.green.set_low();
    }

    fn yellow_active(&mut self) {
        self.red.set_low();
        self.yellow.set_high();
        self.green.set_low();
    }

    fn green_active(&mut self) {
        self.red.set_low();
        self.yellow.set_low();
        self.green.set_high();
    }

    pub fn update(&mut self, adc: &mut ah::Adc) {
        match self.level.analog_read(adc) {
            WaterLevel::THRESHOLD_HIGH.. => self.green_active(),
            WaterLevel::THRESHOLD_LOW..=WaterLevel::THRESHOLD_MEDIUM => self.yellow_active(),
            _ => self.red_active(),
        };
    }
}

#[cfg(not(test))]
#[ah::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = ah::pins!(dp);
    let mut adc = ah::Adc::new(dp.ADC, Default::default());
    let adc2 = pins.a2.into_analog_input(&mut adc);
    pins.d13.into_output().set_low();

    let mut water_measure = WaterLevel::new(pins.d12, pins.d11, pins.d10, adc2);

    let mut serial = ah::default_serial!(dp, pins, 115200);
    
    
    let soil_moisture = pins.a1.into_analog_input(&mut adc);

    loop {
        ah::delay_ms(1000);
        water_measure.update(&mut adc);
        if let SoilState::DRY = get_soil_state(&soil_moisture, &mut adc) {
            water_the_plant();
            ufmt::uwriteln!(&mut serial, "Water the plant").unwrap();
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