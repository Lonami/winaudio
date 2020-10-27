use std::io::{self, Read};
use winapi::um::mmsystem::WAVEHDR;

/// Prepared buffer (header and data) that can be sent to an output device.
pub struct Buffer {
    pub(crate) header: WAVEHDR,
    pub(crate) buffer: Box<[u8]>,
}

impl Buffer {
    /// Reads the next chunk of data into the memory buffer. Returns `false` if not all data was
    /// filled, meaning that the end of the stream has been reached and no more data can be read.
    pub fn read<R: Read>(&mut self, stream: &mut R) -> io::Result<bool> {
        let read = stream.read(&mut self.buffer)?;
        self.buffer[read..].iter_mut().for_each(|x| *x = 0);
        self.header.dwBufferLength = read as u32;
        Ok(read == self.buffer.len())
    }
}
