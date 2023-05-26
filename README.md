# `ht32-usbd`

> [usb-device](https://github.com/rust-embedded-community/usb-device) implementation for Holtek
HT32 microcontrollers.

This project is heavily inspired by the [stm32-usbd](https://github.com/stm32-rs/stm32-usbd)
reference implementation.

It has been primarily developed for, and tested with, the `HT32F1yyy` family of chips,
but should also work with other Holtek chips that use the same USB peripheral.

## Usage

This driver is intended for use through a device hal library.
Such hal library should implement `UsbPeripheral` for the corresponding USB peripheral object.
This trait declares all the peripheral properties that may vary from one device family to the other.

Note that, besides including this library, it is also necessary to make sure that the USB peripheral
is enabled, the USB clock has the right frequency (48MHz) and the backup domain is accessible.

## Examples

See the [ht32f1yyy-hal](https://github.com/ht32-rs/ht32f1yyy-hal) repo for an example of how to 
use this library.
