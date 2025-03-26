#![no_std]
#![no_main]

use defmt::*;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c as I2cTrait;
use {defmt_rtt as _, panic_probe as _};

// For this example we are using the Adafruit ICM-20948 IMU
// connected to the on board Qwiic connector
const ADDRESS: u8 = 0x69;
const WHOAMI: u8 = 0x00;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, Hertz(100_000), Default::default());
    let mut i2c = BlockingAsync::new(i2c);

    // Enable the 3.3v regulator to provide power to the Qwiic connected device
    let _enable_3v3 = Output::new(p.PH0, Level::High, Speed::Low);
    // Allow some time for the i2c device to power on
    Timer::after_millis(100).await;

    let mut data = [0u8; 1];
    unwrap!(i2c.write_read(ADDRESS, &[WHOAMI], &mut data).await);
    info!("Whoami: {:#X}", data[0]);
}
