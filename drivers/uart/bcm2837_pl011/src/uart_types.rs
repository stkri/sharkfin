#[derive(Clone, Copy)]
pub struct UART;

pub type UARTResult<T> = Result<T, UARTError>;

#[derive(Debug)]
pub enum UARTError {}
