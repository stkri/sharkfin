//! Collection of types for the BCM2837 GPIO interface.

/// Different hardware error cases.
pub enum GPIOError {
    /// The pin was not configured properly with GPFSEL.
    WrongFunction,
    /// The pin has no pull and therefore can't produce defined results.
    NoPull,
    /// The pin number exceeds 53.
    NonExistentPin,
    /// The pin cannot be converted to the alternative mode, as it is reserved.
    ReservedPinFunction,
    /// The pin cannot be converted to the alternative mode, as it is internal.
    InternalPinFunction,
    /// The pin cannot be converted to the alternative mode, as it is not defined.
    UndefinedPinFunction,
    // TODO: Add more GPIO error cases.
}

/// Default pin state as configured by the pull resistors.
pub enum GPIOPull {
    /// There is no pull. Can't be used other than for Output.
    None,
    /// The pin is set high per default.
    Up,
    /// The pin is set low per default.
    Down,
}

/// Type corresponding to input (`000`).
pub struct GPIOIn {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to output (`001`).
pub struct GPIOOut {
    pub pin: u8,
    pub pull: GPIOPull,
}

// TODO: Document each pins alternate functions
/// Type corresponding to ALT0 (`100`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt0 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT1 (`101`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt1 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT2 (`110`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt2 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT3 (`111`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt3 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT4 (`011`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt4 {
    pub pin: u8,
    pub pull: GPIOPull,
}

/// Type corresponding to ALT5 (`010`).
/// Must be converted to the proper peripheral type to be used.
pub struct GPIOAlt5 {
    pub pin: u8,
    pub pull: GPIOPull,
}