#![no_std]

//! This crate provides traits for different hardware interfaces.
//! Always use these traits for drivers.
pub mod gpio_pin;
pub mod sys_clock;
pub mod uart;
