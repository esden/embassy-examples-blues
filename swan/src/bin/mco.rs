#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{Mco, McoPrescaler, McoSource};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // NOTE: the MCO pin is not routed to any pins on the Swan board
    // we are still leaving this example in for reference
    let _mco = Mco::new(p.MCO, p.PA8, McoSource::HSI, McoPrescaler::DIV1);

    let mut led = Output::new(p.PE2, Level::High, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
