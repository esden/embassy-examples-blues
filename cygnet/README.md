# Rust Embassy Blues Cygnet Examples

This project contains small example projects implemented using [embassy](https://embassy.dev/) async framework. They are based on the official examples for the stm32l4 microcontroller and slightly adapted to the Cygnet board.

We keep all the examples here even if they are not possible to use on the Cygnet.

You can find the [Cygnet board in the Blues Wireless Store](https://shop.blues.com/products/cygnet). The board schematics can be found in the [GitHub repository](https://github.com/blues/note-hardware/tree/master/Cygnet/v1.2);

# Notes

Not all examples are fully tested on the Swan due to lack of hardware. Here is a list of tested examples as of 2025-03-26:

* [x] adc
* [x] blinky
* [x] button
* [x] button_exti
* [ ] can
* [ ] dac
* [ ] dac_dma
* [x] i2c
* [x] i2c_blocking_async
* [x] i2c_dma
* [x] mco
* [x] rng
* [x] rtc
* [ ] spe_adin1110_http_server
* [ ] spi
* [ ] spi_blocking_async
* [ ] spi_dma
* [ ] tsc_async
* [ ] tsc_blocking
* [ ] tsc_multipin
* [x] usart
* [x] usart_dma
* [x] usb_serial

# Contributions

Additional example contributions are very welcome. Feel free to open pull requests with additional examples.
