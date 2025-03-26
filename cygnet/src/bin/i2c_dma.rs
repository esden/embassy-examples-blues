#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// For this example we are using the Adafruit ICM-20948 IMU
// connected to the on board Qwiic connector
const ADDRESS: u8 = 0x69;
const WHOAMI: u8 = 0x00;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Hertz(100_000),
        Default::default(),
    );

    // Enable the 3.3v regulator to provide power to the Qwiic connected device
    let _enable_3v3 = Output::new(p.PH0, Level::High, Speed::Low);
    // Allow some time for the i2c device to power on
    Timer::after_millis(100).await;

    let mut data = [0u8; 1];
    unwrap!(i2c.write_read(ADDRESS, &[WHOAMI], &mut data).await);
    info!("Whoami: {:#X}", data[0]);
}
