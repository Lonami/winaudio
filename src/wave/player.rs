use crate::device::WAVE_MAPPER;
use crate::wave::{Format, Out};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// Helper to play `.wav` files.
pub struct Player {
    fmt: Format,
    file: File,
}

impl Player {
    /// Creates a new `Player` instance from a `.wav` file stored on disk.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data_id = [0; 4];
        // Check if the file is a RIFF WAVE file.
        // https://web.archive.org/web/20101208013508/http://www.it.fht-esslingen.de/~schmidt/vorlesungen/mm/seminar/ss00/HTML/node128.html
        file.read_exact(&mut data_id)?;
        if &data_id != b"RIFF" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected file format",
            ));
        }
        file.seek(SeekFrom::Start(0))?;
        const OFFSET_FMT_LENGTH: u64 = 4;
        // https://web.archive.org/web/20101207175128/http://www.it.fht-esslingen.de/~schmidt/vorlesungen/mm/seminar/ss00/HTML/node130.html
        let offset = match Self::find_string_in_file(&mut file, "fmt ") {
            Ok(offset) => offset + OFFSET_FMT_LENGTH,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unexpected file format: {}", e),
                ))
            }
        };

        let fmt = Format::from_wav_stream(&mut file, offset)?;

        file.seek(SeekFrom::Start(0))?;

        Ok(Self { fmt, file })
    }

    /// Seeks in an open binary file for the first occurrence of a certain string.
    /// Reads chunks of at most 512 bytes and returns the index after the found string.
    fn find_string_in_file(file: &mut File, target: &str) -> io::Result<u64> {
        let needle = target.as_bytes();
        let mut haystack = [0; 512];
        let mut offset = 0;

        loop {
            let haystack_size = file.read(&mut haystack)?;
            if haystack_size == 0 {
                break;
            }

            if let Some(pos) = haystack[..haystack_size]
                .windows(needle.len())
                .position(|window| window == needle)
            {
                return Ok(offset + pos as u64 + needle.len() as u64);
            }

            // subtract needle length in case the needle is split between two chunks
            offset += haystack_size as u64 - needle.len() as u64;
            file.seek(SeekFrom::Start(offset))?;
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "string not found"))
    }

    /// Sets the volume of the audio device.
    /// The volume is a value between 0.0 and 1.0.
    /// Returns the previous volume setting.
    pub fn set_volume(&mut self, left: f32, right: f32) -> io::Result<(f32, f32)> {
        let mut device = Out::open(WAVE_MAPPER, &self.fmt).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to open output audio device: {:?}", e),
            )
        })?;
        let current_volume = device.get_volume().unwrap();
        device.set_volume(left, right).unwrap();
        Ok(current_volume)
    }
    /// Plays the file from beginning to end.
    pub fn play(&mut self) -> io::Result<()> {
        let mut device = Out::open(WAVE_MAPPER, &self.fmt).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to open output audio device: {:?}", e),
            )
        })?;

        let mut buf_idx = false;
        loop {
            let full = device.buffers()[buf_idx as usize].read(&mut self.file)?;
            match buf_idx {
                false => {
                    device.write_first().map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "failed to write first buffer")
                    })?;
                }
                true => {
                    device.write_second().map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "failed to write second buffer")
                    })?;
                }
            }
            buf_idx = !buf_idx;
            if !full {
                break;
            }
        }

        Ok(())
    }
}
