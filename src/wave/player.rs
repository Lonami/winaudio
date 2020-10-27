use crate::device::WAVE_MAPPER;
use crate::util::BinaryRead as _;
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
    /// Create a new `Player` instance from a `.wav` file stored in disk.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;

        let fmt = Format::from_wav_stream(&mut file)?;

        let file_len = file.seek(SeekFrom::End(0))?;

        const WF_OFFSET_DATA_SUBCHUNK: u64 = 36;
        file.seek(SeekFrom::Start(WF_OFFSET_DATA_SUBCHUNK))?;

        let mut data_id = [0; 4];
        file.read(&mut data_id)?;
        if &data_id != b"data" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected data subchunk id",
            ));
        }
        let meta_data_length = file.read_u32().unwrap() as u64;

        const WF_OFFSET_DATA: u64 = 44;
        let data_length = file_len - WF_OFFSET_DATA;

        if meta_data_length > data_length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "format data length was greater than actual file length",
            ));
        }

        Ok(Self { fmt, file })
    }

    /// Play the file from beginning to end.
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
