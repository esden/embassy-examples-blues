#![no_std]
#![no_main]

use defmt::*;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use embedded_hal_async::spi::SpiBus;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(1_000_000);

    let spi = Spi::new_blocking(p.SPI1, p.PA5, p.PB5, p.PA6, spi_config);

    let mut spi = BlockingAsync::new(spi);

    // These are the pins for the Inventek eS-Wifi SPI Wifi Adapter.

    let _boot = Output::new(p.PB12, Level::Low, Speed::VeryHigh);
    let _wake = Output::new(p.PB13, Level::Low, Speed::VeryHigh);
    let mut reset = Output::new(p.PA7, Level::Low, Speed::VeryHigh);
    let mut cs = Output::new(p.PB1, Level::High, Speed::VeryHigh);
    let ready = Input::new(p.PA3, Pull::Up);

    cortex_m::asm::delay(100_000);
    reset.set_high();
    cortex_m::asm::delay(100_000);

    while ready.is_low() {
        info!("waiting for ready");
    }

    let write: [u8; 10] = [0x0A; 10];
    let mut read: [u8; 10] = [0; 10];
    cs.set_low();
    spi.transfer(&mut read, &write).await.ok();
    cs.set_high();
    info!("xfer {=[u8]:x}", read);
}
