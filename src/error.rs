use core::fmt::{Debug, Display};

/// The error type used by this library.
///
/// This can encapsulate an SPI or GPIO error, and adds its own protocol errors
/// on top of that.
#[derive(Debug)]
pub enum Error<SpiError> {
    /// An SPI transfer failed.
    Spi(SpiError),

    /// Status register contained unexpected flags.
    ///
    /// This can happen when the chip is faulty, incorrectly connected, or the
    /// driver wasn't constructed or destructed properly (eg. while there is
    /// still a write in progress).
    UnexpectedStatus,

    /// Received an invalid response from the flash memory.
    InvalidResponse,
}

impl<SpiError: Display> Display for Error<SpiError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Spi(e) => write!(f, "SPI error: {e}"),
            Error::UnexpectedStatus => f.write_str("unexpected value in status register"),
            Error::InvalidResponse => f.write_str("invalid response"),
        }
    }
}
