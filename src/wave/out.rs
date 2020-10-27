use crate::util::{check_multimedia_error, Event};
use crate::wave::{Buffer, Format};
use crate::Error;
use std::mem;
use std::pin::Pin;
use std::ptr;
use winapi::um::mmeapi::{
    waveOutClose, waveOutOpen, waveOutPause, waveOutPrepareHeader, waveOutRestart,
    waveOutSetVolume, waveOutUnprepareHeader, waveOutWrite, waveOutReset,
};
use winapi::um::mmsystem::{CALLBACK_FUNCTION, HWAVEOUT, WAVEHDR, WOM_CLOSE, WOM_DONE, WOM_OPEN};
use winapi::um::winnt::LPSTR;

const HDR_SIZE: u32 = mem::size_of::<WAVEHDR>() as u32;
const WHDR_PREPARED: u32 = 0x00000002;

extern "C" fn callback(_hwo: HWAVEOUT, msg: u32, instance: usize, _param1: usize, _param2: usize) {
    match msg {
        WOM_OPEN | WOM_CLOSE => {}
        WOM_DONE => {
            let event = unsafe { &mut *(instance as *mut Event) };
            event.set();
        }
        _ => panic!("unexpected callback message"),
    }
}

/// Access to a wave output device.
pub struct Out {
    hwo: HWAVEOUT,
    cb_done: Pin<Box<Event>>,
    // The buffers must remain valid while the device is playing them,
    // and unless we own them they could be dropped at any time.
    // This also means that their lifecycle has to be handled manually.
    buffers: [Buffer; 2],
}

impl Out {
    /// Individual buffer size for each of the two buffers.
    const BUFFER_SIZE: usize = 256 * 1024;

    /// Opens the specified waveform-audio output device for playback.
    ///
    /// The waveform-audio output device identifier is a number in the range `0..device::count()`.
    /// The `device::WAVE_MAPPER` may also be used to automatically select a compatible device.
    pub fn open(device_id: u32, fmt: &Format) -> Result<Self, Error> {
        let cb_done = Box::pin(Event::new());
        (*cb_done).set(); // start ready

        let mut hwo: HWAVEOUT = ptr::null_mut();
        check_multimedia_error(unsafe {
            waveOutOpen(
                &mut hwo,
                device_id,
                &fmt.c_struct(),
                callback as usize,
                &*cb_done as *const Event as usize,
                CALLBACK_FUNCTION,
            )
        })?;

        let align = fmt.block_align as usize;
        let new_block = || match Self::prepare_block(hwo, align, Self::BUFFER_SIZE) {
            Ok(x) => Ok(x),
            Err(e) => {
                unsafe { waveOutClose(hwo) };
                Err(e)
            }
        };
        let first = new_block()?;
        let second = new_block()?;

        Ok(Self {
            hwo,
            cb_done,
            buffers: [first, second],
        })
    }

    /// New volume setting. A value of 1.0 represents full volume, and 0.0 silence.
    ///
    /// If a device does not support both left and right volume control, the
    /// left volume level will be used, and the right volume level is ignored.
    pub fn set_volume(&mut self, left: f32, right: f32) -> Result<(), Error> {
        if left < 0.0 || left > 1.0 || right < 0.0 || right > 1.0 {
            return Err(Error::InvalidParam);
        }
        let left = (left * 0xffff as f32) as u32;
        let right = (right * 0xffff as f32) as u32;
        let vol = left | (right << 16);
        check_multimedia_error(unsafe { waveOutSetVolume(self.hwo, vol) })
    }

    /// Prepares a waveform-audio data block for playback. Data can be read
    /// into the block before sending the block for playback to `write()`.
    fn prepare_block(hwo: HWAVEOUT, align: usize, mut size: usize) -> Result<Buffer, Error> {
        if size % align != 0 {
            size += align - (size % align);
        }

        let mut buffer = vec![0; size].into_boxed_slice();
        let mut header = WAVEHDR {
            lpData: buffer.as_mut_ptr() as LPSTR,
            dwBufferLength: buffer.len() as u32,
            dwBytesRecorded: 0,
            dwUser: 0,
            dwFlags: 0,
            dwLoops: 0,
            lpNext: ptr::null_mut(),
            reserved: 0,
        };
        check_multimedia_error(unsafe { waveOutPrepareHeader(hwo, &mut header, HDR_SIZE) })?;

        if header.dwFlags & WHDR_PREPARED == 0 {
            return Err(Error::InvalidFlag);
        }
        Ok(Buffer { header, buffer })
    }

    /// Get a mutable reference to the buffers so that they can be read into.
    pub fn buffers(&mut self) -> &mut [Buffer; 2] {
        &mut self.buffers
    }

    /// Write the data block from the first buffer to the waveform-audio output device.
    ///
    /// Note that this has to `wait` until the previous buffer completes, and will lock
    /// indefinitely if the playback is paused.
    pub fn write_first(&mut self) -> Result<(), Error> {
        self.wait();
        self.cb_done.clear();
        check_multimedia_error(unsafe {
            waveOutWrite(self.hwo, &mut self.buffers[0].header, HDR_SIZE)
        })
    }

    /// Write the data block from the second buffer to the waveform-audio output device.
    ///
    /// Note that this has to `wait` until the previous buffer completes, and will lock
    /// indefinitely if the playback is paused.
    pub fn write_second(&mut self) -> Result<(), Error> {
        self.wait();
        self.cb_done.clear();
        check_multimedia_error(unsafe {
            waveOutWrite(self.hwo, &mut self.buffers[1].header, HDR_SIZE)
        })
    }

    /// Wait for the device to finish playing the last chunk of data written.
    pub fn wait(&self) {
        self.cb_done.wait();
    }

    /// Pauses playback on the output device. The current position is saved.
    ///
    /// Calling this function when the output is already paused has no effect,
    /// and the function returns `Ok`.
    pub fn pause(&mut self) -> Result<(), Error> {
        check_multimedia_error(unsafe { waveOutPause(self.hwo) })
    }

    /// Resume playback on the paused output device.
    ///
    /// Calling this function when the output is not paused has no effect, and the function
    /// returns `Ok`.
    pub fn resume(&mut self) -> Result<(), Error> {
        check_multimedia_error(unsafe { waveOutRestart(self.hwo) })
    }

    /// Stops playback on the output device and resets the current position to zero. All
    /// pending playback buffers are marked as done.
    pub fn stop(&mut self) -> Result<(), Error> {
        check_multimedia_error(unsafe { waveOutReset(self.hwo) })
    }
}

impl Drop for Out {
    fn drop(&mut self) {
        // TODO leak buffers instead of panicking
        self.stop().expect("failed to stop playback prior to drop");

        let hwo = self.hwo;

        // Can't do this in the buffers' drop because we own them and would be
        // dropped after dropping self (when the device handle is already closed).
        self.buffers.iter_mut().for_each(|b| {
            if b.header.dwFlags & WHDR_PREPARED != 0 {
                match check_multimedia_error(unsafe {
                    waveOutUnprepareHeader(hwo, &mut b.header, HDR_SIZE)
                }) {
                    Ok(_) => {}
                    Err(e) => eprintln!("error during unprepare header: {:?}", e),
                }
            }
        });

        match check_multimedia_error(unsafe { waveOutClose(hwo) }) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error dropping wave out handle: {:?}", e);
            }
        }
    }
}
