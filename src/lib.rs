#![cfg(windows)]
//! This crate serves as an abstraction over the audio functions provided by the
//! [Windows Multimedia API]. In particular, it enables its users to enumerate the output devices,
//! list their capabilities, and open them for playback. Basically, it lets you play sound files
//! in Windows:
//!
//! ```no_run
//! use winaudio::wave::Player;
//!
//! fn main() {
//!     let mut player = Player::from_file("test.wav").unwrap();
//!     player.play().unwrap();
//! }
//! ```
//! 
//! If you're missing a certain function from the [mmeapi.h header], feel free to open an issue
//! or send a pull request to the project to add it. This initial version doesn't have methods to
//! set the pitch or playback rate for example, but they can trivially be added if needed.
//!
//! [Windows Multimedia API]: https://docs.microsoft.com/en-us/windows/win32/api/_multimedia/
//! [mmeapi.h header]: https://docs.microsoft.com/en-us/windows/win32/api/mmeapi/

#[macro_use]
mod util;
pub mod device;
mod error;
pub mod wave;

pub use error::Error;
