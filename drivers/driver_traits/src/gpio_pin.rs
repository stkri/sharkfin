//! GPIO pin abstractions.
//!
//! **If you are using a driver, avoid using these functions.**\
//! These functions are intended to be used inside drivers that access the GPIO.
//! These drivers should have way better functions for your purposes.
//! While some of these functions aren't unsafe, **if you do not know what you are doing,**
//! **you will end up with garbage data.**
/// Abstraction over GPIO pin which can accept input.
///
/// **If you are using a driver, avoid using these functions.**\
/// Check the driver for convenient read functions.
pub trait InputPin {
    type Error;
    /// Checks if pin voltage is near ground.
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    fn set_low(&self) -> Result<bool, Self::Error>;
    /// Checks if pin voltage is near VCC.
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    fn set_high(&self) -> Result<bool, Self::Error>;
}
/// Abstraction over GPIO pin which can accept output.
///
/// **If you are using a driver, avoid using these functions.**\
/// Check the driver for safe write functions.
pub trait OutputPin {
    type Error;
    /// Sets pin voltage to Ground
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    ///
    /// # Safety
    /// Hardware write operations are inherently unsafe.
    /// Make sure hardware addresses are valid, and the pin is properly set up.
    unsafe fn set_low(&mut self) -> Result<(), Self::Error>;
    /// Sets pin voltage to VCC
    ///
    /// # Errors
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    ///
    /// # Safety
    /// Hardware write operations are inherently unsafe.
    /// Make sure hardware addresses are valid, and the pin is properly set up.
    unsafe fn set_high(&mut self) -> Result<(), Self::Error>;
}
/// Extension to OutputPin abstraction.
/// Guarantees a possibility to check the set pin state.
///
/// **If you are using a driver, avoid using these functions.**\
/// Check the driver for safe write and read functions.
pub trait StatefulOutputPin: OutputPin {
    /// Check if the pin voltage is set to ground.
    fn is_set_low(&self) -> bool;
    /// Check if the pin voltage is set to VCC.
    fn is_set_high(&self) -> bool;
    /// Toggle pin voltage.
    ///
    /// # Error
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    ///
    /// # Safety
    /// Hardware write operations are inherently unsafe.
    /// Make sure hardware addresses are valid, and the pin is properly set-up.
    unsafe fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Abstraction to add pull configuration to GPIO pin.
pub trait ConfigurablePull {
    type Error;
    type Pull;
    /// Set the pin pull to chosen option.
    ///
    /// # Error
    /// Fails if the hardware operation fails.
    /// Check driver documentation for more information.
    ///
    /// # Safety
    /// Hardware operations are inherently unsafe.
    /// Make sure the hardware addresses are valid, and the timing is correct.
    unsafe fn set_pull(&mut self, pull: Self::Pull) -> Result<(), Self::Error>;
}