#![no_std]
#![no_main]

/**  RUSTY ROBOT ROVER
https://wronganswer.blog/rustcar/

TODO

*/
use arduino_hal::hal::port::{PB3, PB4};
use arduino_hal::pac::TC1;
use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;

use arduino_hal::Delay;

struct SuperSonicSensor {
    timer: TC1,
    trigger: Pin<Output, PB4>,
    echo: Pin<Input<Floating>, PB3>,
}

impl SuperSonicSensor {
    fn get_distance(&mut self) -> u16 {
        let mut delay = Delay::new();
        self.timer.tcnt1.write(|w| w.bits(0));
        self.trigger.set_high();
        delay.delay_us(10u16);
        self.trigger.set_low();

        while self.echo.is_low() {
            if self.timer.tcnt1.read().bits() >= 6500 {
                return 63500;
            }
        }

        self.timer.tcnt1.write(|w| w.bits(0));
        while self.echo.is_high() {}

        (self.timer.tcnt1.read().bits() * 4) / 58
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!").void_unwrap();

    let timer = dp.TC1;
    timer.tccr1b.write(|w| w.cs1().prescale_64());

    let mut sonic_sensor = SuperSonicSensor {
        timer,
        trigger: pins.d12.into_output(),
        echo: pins.d11,
    };

    loop {
        let distance = sonic_sensor.get_distance();

        ufmt::uwriteln!(&mut serial, "{} cm", distance).void_unwrap();
        arduino_hal::delay_ms(100);
    }
}
