//! Trait implementations for GPIOError.

pub use crate::gpio_types::GPIOError;
use core::error::Error;
use core::fmt::Display;
use core::fmt::Formatter;

impl Display for GPIOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            GPIOError::NonExistentPin => write!(f, "The pin does not exist on the GPIO."),
            GPIOError::NoPull => write!(f, "Attempted to use a GPIO without a pull state."),
            GPIOError::UnknownReadError => write!(f, "Unknown error when reading GPIO."),
            GPIOError::UnknownWriteError => write!(f, "Unknown error when writing GPIO."),
            GPIOError::InternalPinFunction => {
                write!(f, "The chosen pin is used for an internal function")
            }
            GPIOError::ReservedPinFunction => {
                write!(f, "The chosen pin is used for a reserved function")
            }
            GPIOError::UndefinedPinFunction => write!(f, "No defined function for the chosen pin"),
            GPIOError::WrongFunction => write!(
                f,
                "The GPIO pin was not configured properly for the function"
            ),
        }
    }
}

// TODO: Implement the provided methods, maybe.
impl Error for GPIOError {}
