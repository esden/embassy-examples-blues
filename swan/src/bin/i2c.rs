#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};

// For this example we are using the Adafruit ICM-20948 IMU
// connected to the on board Qwiic connector
const ADDRESS: u8 = 0x69;
const WHOAMI: u8 = 0x00;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, Hertz(100_000), Default::default());

    let mut data = [0u8; 1];
    unwrap!(i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data));
    info!("Whoami: {:#X}", data[0]);
}
