#![no_std]
#![no_main]

extern crate cortex_m;

extern crate kicksat as hal;
extern crate panic_halt;
//extern crate panic_itm;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{entry, CorePeripherals, Peripherals, gpio};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.led.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let t = 250;

    loop {
        flash(&mut red_led, &mut delay, t);
        delay.delay_ms(500u32);
        flash(&mut red_led, &mut delay, t);
        delay.delay_ms(500u32);
        flash(&mut red_led, &mut delay, t);

        delay.delay_ms(1000u32);
    }
}

fn flash(led: &mut gpio::Pb3<gpio::Output<gpio::OpenDrain>>, delay: &mut Delay, t: u32) {
    led.set_high();
    delay.delay_ms(t);
    led.set_low();
}