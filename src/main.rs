#![cfg_attr(not(test), no_std)]

#![cfg_attr(not(test), no_main)]

use ah::Peripherals;
#[cfg(not(test))]
use panic_halt as _;
use arduino_hal as ah;
use ah::port::{Pin, mode::{Output, Input, Floating, Analog}};
use ah::hal::port::PB4 as LED1;
use ah::hal::port::PB3 as LED2;
use ah::hal::port::PB2 as LED3;
use ah::hal::port::PC1 as ADC1;
use ah::hal::port::PC2 as ADC2;
use ah::hal::port::PC3 as PUMP;
use ah::delay_ms;

const PUMP_ACTIVE_TIME : u16 = 10000;    // ms
const WATER_DELAY_TIME : u16 = 60000;    // ms

struct WaterLevel {
    red : Pin<Output, LED1>,
    yellow : Pin<Output, LED2>,
    green : Pin<Output, LED3>,
    level : Pin<Analog, ADC2>,
    empty : bool,
}

#[derive(PartialEq)]
enum StateMachine {
    AddWater,
    Idle,
    OutOfWater,
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

fn water_the_plant(pump_pin: &mut Pin<Output, PUMP>) {
    pump_pin.set_high();
    delay_ms(PUMP_ACTIVE_TIME);
    pump_pin.set_low();
}

impl WaterLevel {
    const THRESHOLD_HIGH : u16 = 550;
    const THRESHOLD_MEDIUM : u16 = WaterLevel::THRESHOLD_HIGH-1;
    const THRESHOLD_LOW : u16 = 400;

    fn new(red : Pin<Input<Floating>, LED1>, yellow : Pin<Input<Floating>, LED2>,
           green : Pin<Input<Floating>, LED3>, analog_in: Pin<Analog, ADC2>) -> WaterLevel {
            WaterLevel{red: red.into_output(), yellow: yellow.into_output(), 
                green: green.into_output(), level: analog_in, empty: false}
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

        self.empty = match self.level.analog_read(adc) {
            WaterLevel::THRESHOLD_HIGH.. => { self.green_active(); false },
            WaterLevel::THRESHOLD_LOW..=WaterLevel::THRESHOLD_MEDIUM => { self.yellow_active(); false },
            _ => { self.red_active(); true },
        };
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }
}

#[cfg(not(test))]
#[ah::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = ah::pins!(dp);
    let mut adc = ah::Adc::new(dp.ADC, Default::default());
    let adc2 = pins.a2.into_analog_input(&mut adc);
    let mut pump_pin = pins.a3.into_output();
    let mut water_measure = WaterLevel::new(pins.d12, pins.d11, pins.d10, adc2);    
    
    let soil_moisture = pins.a1.into_analog_input(&mut adc);
    water_measure.update(&mut adc);

    let mut state = StateMachine::Idle;
    loop {
        if water_measure.is_empty() {
            state = StateMachine::OutOfWater;
        }
        if state == StateMachine::AddWater {
            water_the_plant(&mut pump_pin);
            water_measure.update(&mut adc);
        }
        
        delay_ms(WATER_DELAY_TIME);
        state = match get_soil_state(&soil_moisture, &mut adc) {
            SoilState::DRY => StateMachine::AddWater,
            SoilState::MIDDLE => if state == StateMachine::AddWater { StateMachine::AddWater }
                                 else {StateMachine::Idle },
            SoilState::WET => StateMachine::Idle
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_check() {
        assert!(true);
    }
}