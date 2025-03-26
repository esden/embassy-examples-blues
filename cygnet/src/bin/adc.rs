#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::adc::{Adc, Resolution};
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Hello World!");

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux.adcsel = mux::Adcsel::SYS;
    }
    let p = embassy_stm32::init(config);

    let mut adc = Adc::new(p.ADC1);
    adc.enable_vrefint();
    adc.set_resolution(Resolution::BITS8);
    // Measuring the +VMAIN
    let mut channel = p.PA4;

    loop {
        let v = adc.blocking_read(&mut channel);
        // Our reference voltage is VIO which is 3.3V by default.
        let v_meas = v as f64 * (3.3 / u8::MAX as f64);
        // +VMAIN has a R1 = 10MOhm and R2 = 4.3MOhm voltage divider
        let voltage = (v_meas as f64 * (10.0 + 4.3)) / 4.3;
        info!("--> {} {}V", v, voltage);
    }
}
