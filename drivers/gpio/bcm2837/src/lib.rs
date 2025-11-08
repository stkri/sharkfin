#![no_std]
//! This is the GPIO driver for the BroadComm BCM2837.
//! The BroadComm GPIO provides 54 pins, which all support input and
//! output and can support up to 6 alternate functions.
//!
//! The alternate functions are handled by external drivers, but you have to
//! initialize the pin through here.

pub mod common_functions;
pub mod gpio_addresses;
pub mod gpio_types;
pub mod impls;
