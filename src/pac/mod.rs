//! Handcrafted PAC for the USB peripheral registers
//!
//! Here the endpoint registers have a *unified interface*, to allow a more elegant implementation
//! of the [`UsbBus`](crate::bus::UsbBus).
//!
//! It is important to keep in mind that in reality not every endpoint accepts every field that has
//! been defined here.
//! For instance the *SETUP Data Received* interrupt is only applicable to endpoint 0,
//! whereas the *Single-Buffering or Double-Buffering Selection* bit is only valid for endpoint 4-7.

#[allow(unused, non_camel_case_types)]
pub mod usb;

#[allow(dead_code)]
mod generic;
