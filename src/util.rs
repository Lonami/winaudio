//! Several utilities to make implementing this crate less cumbersome.
use crate::Error;
use std::convert::TryFrom;
use std::io::{self, Read};
use std::sync::{Condvar, Mutex};
use winapi::um::mmsystem::MMSYSERR_NOERROR;

/// Automatically implement `TryFrom<primitive>` for enumerations with `#[repr(primitive)]`.
/// It also derives the traits that are most desirable for these types of enumerations.
macro_rules! enum_with_try_from {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident($ty:ident) {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $variant:ident = $value:expr
            ),*
            $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr($ty)]
        $(#[$outer])*
        $vis enum $name {
            $(
                $(#[$inner $($args)*])*
                $variant = $value
            ),*
        }

        impl std::convert::TryFrom<$ty> for $name {
            type Error = $ty;

            #[allow(non_upper_case_globals)]
            fn try_from(value: $ty) -> Result<Self, $ty> {
                $(
                    const $variant: $ty = $value;
                )*
                Ok(match value {
                    $(
                        $variant => $name::$variant,
                    )*
                    _ => return Err(value),
                })
            }
        }
    };
}

/// Helper trait to read little-endian integers from binary data.
pub(crate) trait BinaryRead: Read {
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.read(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl<T> BinaryRead for T where T: Read {}

/// Convert the error code into the proper `Error` variant.
pub(crate) fn check_multimedia_error(result: u32) -> Result<(), Error> {
    if result == MMSYSERR_NOERROR {
        Ok(())
    } else {
        Err(Error::try_from(result).expect("unexpected multimedia error"))
    }
}

/// Helper struct to store an event flag and the condition variable to wait on it.
pub(crate) struct Event {
    mutex: Mutex<bool>,
    cond: Condvar,
}

impl Event {
    /// Create a new instance of the event, initially clear.
    pub(crate) fn new() -> Self {
        Self {
            mutex: Mutex::new(false),
            cond: Condvar::new(),
        }
    }

    /// Set the event. This will wake up everyone `wait`ing on it.
    pub(crate) fn set(&self) {
        let mut guard = self.mutex.lock().unwrap();
        *guard = true;
        self.cond.notify_all();
    }

    /// Clear the event. Calls to `wait` will block until it's `set`.
    pub(crate) fn clear(&self) {
        let mut guard = self.mutex.lock().unwrap();
        *guard = false;
    }

    /// Wait for the event to be `set`.
    pub(crate) fn wait(&self) {
        let mut guard = self.mutex.lock().unwrap();
        while !*guard {
            guard = self.cond.wait(guard).unwrap();
        }
    }
}
