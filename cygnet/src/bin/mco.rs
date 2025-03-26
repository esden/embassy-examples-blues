#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::rcc::{Mco, McoPrescaler, McoSource};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // NOTE: the MCO pin is connected to the bulitin LED
    let _mco = Mco::new(p.MCO, p.PA8, McoSource::HSI, McoPrescaler::DIV1);

    loop {
        info!("ping");
        Timer::after_millis(300).await;
        info!("pong");
        Timer::after_millis(300).await;
    }
}
