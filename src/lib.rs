//! USB peripheral driver for HT32 microcontrollers.

#![no_std]

pub mod bus;
mod endpoint;
mod endpoint_memory;
mod registers;
pub use crate::bus::UsbBus;

mod pac;

/// A trait for device-specific USB peripherals. Implement this to add support for a new hardware
/// platform. Peripherals that have this trait must have the same register block as the USBFS
/// peripheral in the HT32F1yyy series.
///
/// # Safety
/// TODO
pub unsafe trait UsbPeripheral: Send + Sync {
    /// Pointer to the register block
    const REGISTERS: *const ();

    /// Internal pull-up resistor on USB_DP line
    const DP_PULL_UP_FEATURE: bool;

    /// Pointer to the endpoint memory
    const EP_MEMORY: *const ();

    /// Endpoint memory size in bytes
    const EP_MEMORY_SIZE: usize;

    /// Enables USB device on its peripheral bus
    fn enable(&self);

    /// Enable the external pull-up resistor for the USB_DP line.
    /// Required when `DP_PULL_UP_FEATURE` is false.
    fn dp_pull_up(&mut self);
}
