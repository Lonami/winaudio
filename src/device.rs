//! Functions to retrieve device count and capabilities.
//!
//! ```
//! use winaudio::device;
//!
//! println!("There are {} device(s).", device::count());
//!
//! for dev in 0..device::count() {
//!     println!("Device {} capabilities: {:#?}",
//!              dev, device::get_capabilities(dev).unwrap());
//! }
//! ```
use crate::util::check_multimedia_error;
use crate::Error;
use std::convert::TryFrom;
use std::fmt;
use std::mem::{self, MaybeUninit};
use widestring::U16CString;
use winapi::um::mmeapi::*;
use winapi::um::mmsystem::*;

/// Used to select a waveform-audio output device capable of playing the given format.
pub use winapi::um::mmsystem::WAVE_MAPPER;

// Constants from "shared/mmreg.h".
enum_with_try_from!(
/// Product identifier for a device.
pub enum Product(u16) {
    /// Adlib-compatible synthesizer.
    Adlib = 9,
    /// G.711 codec.
    AcmG711 = 37,
    /// GSM 610 codec.
    AcmGsm610 = 36,
    /// IMA ADPCM codec.
    ImaAdpcm = 34,
    /// Joystick adapter.
    Joystick = 12,
    /// MIDI mapper.
    Midi = 1,
    /// MPU 401-compatible MIDI input port.
    Mpu401MidiIn = 11,
    /// MPU 401-compatible MIDI output port.
    Mpu401MidiOut = 10,
    /// MS ADPCM codec.
    AcmAdpcm = 33,
    /// MS audio board stereo FM synthesizer.
    FmSynthStereo = 16,
    /// MS audio board aux port.
    Aux = 21,
    /// MS audio board mixer driver.
    Mixer = 17,
    /// MS audio board waveform input.
    WaveIn = 14,
    /// MS audio board waveform output.
    WaveOut = 15,
    /// MS audio compression manager.
    Acm = 32,
    /// MS filter.
    AcmFilter = 35,
    /// MS OEM audio aux port.
    OemAux = 22,
    /// MS OEM audio board mixer driver.
    OemMixer = 31,
    /// MS OEM audio board stereo FM synthesizer.
    OemFmSynthStereo = 20,
    /// MS OEM audio board waveform input.
    OemWaveIn = 18,
    /// MS OEM audio board waveform output.
    OemWaveOut = 19,
    /// MS vanilla driver aux (CD).
    GenericAuxCd = 30,
    /// MS vanilla driver aux (line in).
    GenericAuxLine = 28,
    /// MS vanilla driver aux (mic).
    GenericAuxMic = 29,
    /// MS vanilla driver MIDI external out.
    GenericMidiOut = 26,
    /// MS vanilla driver MIDI in.
    GenericMidiIn = 25,
    /// MS vanilla driver MIDI synthesizer.
    GenericMidiSynth = 27,
    /// MS vanilla driver waveform input.
    GenericWaveIn = 23,
    /// MS vanilla driver wavefrom output.
    GenericWaveOut = 24,
    /// PC speaker waveform output.
    SpeakerWaveOut = 13,
    /// PCM converter.
    AcmPcm = 38,
    /// Sound Blaster internal synthesizer.
    SoundBlasterSynth = 5,
    /// Sound Blaster MIDI input port.
    SoundBlasterMidiIn = 4,
    /// Sound Blaster MIDI output port.
    SoundBlasterMidiOut = 3,
    /// Sound Blaster waveform input.
    SoundBlasterWaveIn = 7,
    /// Sound Blaster waveform output.
    SoundBlasterWaveOut = 6,
    /// Wave mapper.
    WaveMapper = 2,
});

enum_with_try_from!(
/// Manufacturer for the device driver for a device.
pub enum Manufacturer(u16) {
    /// Advanced Gravis Computer Technology, Ltd.
    Gravis = 34,
    /// Antex Electronics Corporation.
    Antex = 31,
    /// APPS Software.
    Apps = 42,
    /// Artisoft, Inc.
    Artisoft = 20,
    /// AST Research, Inc.
    Ast = 64,
    /// ATI Technologies, Inc.
    Ati = 27,
    /// Audio, Inc.
    AudioFile = 47,
    /// Audio Processing Technology.
    Apt = 56,
    /// Audio Processing Technology.
    AudioPt = 74,
    /// Auravision Corporation.
    AuraVision = 80,
    /// Aztech Labs, Inc.
    Aztech = 52,
    /// Canopus, Co., Ltd.
    Canopus = 49,
    /// Compusic.
    Compusic = 89,
    /// Computer Aided Technology, Inc.
    Cat = 41,
    /// Computer Friends, Inc.
    ComputerFriends = 45,
    /// Control Resources Corporation.
    ControlRes = 84,
    /// Creative Labs, Inc.
    Creative = 2,
    /// Dialogic Corporation.
    Dialogic = 93,
    /// Dolby Laboratories, Inc.
    Dolby = 78,
    /// DSP Group, Inc.
    DspGroup = 43,
    /// DSP Solutions, Inc.
    DspSolutions = 25,
    /// Echo Speech Corporation.
    Echo = 39,
    /// ESS Technology, Inc.
    Ess = 46,
    /// Everex Systems, Inc.
    Everex = 38,
    /// EXAN, Ltd.
    Exan = 63,
    /// Fujitsu, Ltd.
    Fujitsu = 4,
    /// I/O Magic Corporation.
    IoMagic = 82,
    /// ICL Personal Systems.
    IclPs = 32,
    /// Ing. C. Olivetti & C., S.p.A.
    Olivetti = 81,
    /// Integrated Circuit Systems, Inc.
    Ics = 57,
    /// Intel Corporation.
    Intel = 33,
    /// InterActive, Inc.
    Interactive = 36,
    /// International Business Machines.
    Ibm = 22,
    /// Iterated Systems, Inc.
    IteratedSys = 58,
    /// Logitech, Inc.
    Logitech = 60,
    /// Lyrrus, Inc.
    Lyrrus = 88,
    /// Matsushita Electric Corporation of America.
    Matsushita = 83,
    /// Media Vision, Inc.
    MediaVision = 3,
    /// Metheus Corporation.
    Metheus = 59,
    /// microEngineering Labs.
    MeLabs = 44,
    /// Microsoft Corporation.
    Microsoft = 1,
    /// MOSCOM Corporation.
    Moscom = 68,
    /// Motorola, Inc.
    Motorola = 48,
    /// Natural MicroSystems Corporation.
    Nms = 87,
    /// NCR Corporation.
    Ncr = 62,
    /// NEC Corporation.
    Nec = 26,
    /// New Media Corporation.
    NewMedia = 86,
    /// OKI.
    Oki = 79,
    /// OPTi, Inc.
    Opti = 90,
    /// Roland Corporation.
    Roland = 24,
    /// SCALACS.
    Scalacs = 54,
    /// Seiko Epson Corporation, Inc.
    Epson = 50,
    /// Sierra Semiconductor Corporation.
    Sierra = 40,
    /// Silicon Software, Inc.
    SiliconSoft = 69,
    /// Sonic Foundry.
    SonicFoundry = 66,
    /// Speech Compression.
    SpeechComp = 76,
    /// Supermac Technology, Inc.
    Supermac = 73,
    /// Tandy Corporation.
    Tandy = 29,
    /// Toshihiko Okuhura, Korg, Inc.
    Korg = 55,
    /// Truevision, Inc.
    Truevision = 51,
    /// Turtle Beach Systems.
    TurtleBeach = 21,
    /// Video Associates Labs, Inc.
    Val = 35,
    /// VideoLogic, Inc.
    VideoLogic = 53,
    /// Visual Information Technologies, Inc.
    Vitec = 67,
    /// VocalTec, Inc.
    VocalTec = 23,
    /// Voyetra Technologies.
    Voyetra = 30,
    /// Wang Laboratories.
    WangLabs = 28,
    /// Willow Pond Corporation.
    WillowPond = 65,
    /// Winnov, LP.
    Winnov = 61,
    /// Xebec Multimedia Solutions Limitedv.
    Xebec = 85,
    /// Yamaha Corporation of America.
    Yamaha = 37,
});

/// Standard device formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Format {
    /// 11.025 kHz, mono, 8-bit.
    Mono8b11Khz = WAVE_FORMAT_1M08,
    /// 11.025 kHz, mono, 16-bit.
    Mono16b11Khz = WAVE_FORMAT_1M16,
    /// 11.025 kHz, stereo, 8-bit.
    Stereo8b11Khz = WAVE_FORMAT_1S08,
    /// 11.025 kHz, stereo, 16-bit.
    Stereo16b11Khz = WAVE_FORMAT_1S16,
    /// 22.05 kHz, mono, 8-bit.
    Mono8b22Khz = WAVE_FORMAT_2M08,
    /// 22.05 kHz, mono, 16-bit.
    Mono16b22Khz = WAVE_FORMAT_2M16,
    /// 22.05 kHz, stereo, 8-bit.
    Stereo8b22Khz = WAVE_FORMAT_2S08,
    /// 22.05 kHz, stereo, 16-bit.
    Stereo16b22Khz = WAVE_FORMAT_2S16,
    /// 44.1 kHz, mono, 8-bit.
    Mono8b44Khz = WAVE_FORMAT_4M08,
    /// 44.1 kHz, mono, 16-bit.
    Mono16b44Khz = WAVE_FORMAT_4M16,
    /// 44.1 kHz, stereo, 8-bit.
    Stereo8b44Khz = WAVE_FORMAT_4S08,
    /// 44.1 kHz, stereo, 16-bit.
    Stereo16b44Khz = WAVE_FORMAT_4S16,
    /// 96 kHz, mono, 8-bit.
    Mono8b96Khz = WAVE_FORMAT_96M08,
    /// 96 kHz, mono, 16-bit.
    Mono16b96Khz = WAVE_FORMAT_96M16,
    /// 96 kHz, stereo, 8-bit.
    Stereo8b96Khz = WAVE_FORMAT_96S08,
    /// 96 kHz, stereo, 16-bit.
    Stereo16b96Khz = WAVE_FORMAT_96S16,
}

/// Additional functionality a device may provide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Functionality {
    /// Supports separate left and right volume control.
    LrVolume = 0x0008,
    /// Supports pitch control.
    Pitch = 0x0001,
    /// Supports playback rate control.
    PlaybackRate = 0x0002,
    /// The driver is synchronous and will block while playing a buffer.
    Sync = 0x0010,
    /// Supports volume control.
    Volume = 0x0004,
    /// Returns sample-accurate position information.
    SampleAccurate = 0x0020,
}

/// Describes the capabilities of a waveform-audio output device.
#[derive(Clone)]
pub struct Capabilities {
    caps: WAVEOUTCAPSW,
}

impl Capabilities {
    /// Manufacturer for the device driver for the device.
    pub fn manufacturer(&self) -> Manufacturer {
        Manufacturer::try_from(self.caps.wMid).expect("unknown manufacturer")
    }

    /// Product identifier for the device.
    pub fn product(&self) -> Option<Product> {
        // The "mmreg.h" header file contains duplicate identifiers for different products, so
        // it's impossible to have a single enumeration with all of them. Instead, only these:
        // https://docs.microsoft.com/en-us/windows/win32/multimedia/microsoft-corporation-product-identifiers
        // ...are included.
        Product::try_from(self.caps.wPid).ok()
    }

    /// Version number of the device driver for the device.
    pub fn driver_version(&self) -> (u8, u8) {
        let major = (self.caps.vDriverVersion >> 8) & 0xff;
        let minor = self.caps.vDriverVersion & 0xff;
        (major as u8, minor as u8)
    }

    /// Product name.
    pub fn name(&self) -> String {
        let unaligned = &raw const self.caps.szPname;
        let raw = unsafe { std::ptr::read_unaligned(unaligned) };
        let name = unsafe { U16CString::from_ptr_str(raw.as_ptr()) };
        name.to_string().expect("non-utf8 product name")
    }

    /// Standard formats that are supported.
    pub fn supported_formats(&self) -> Vec<Format> {
        [
            Format::Mono8b11Khz,
            Format::Mono16b11Khz,
            Format::Stereo8b11Khz,
            Format::Stereo16b11Khz,
            Format::Mono8b22Khz,
            Format::Mono16b22Khz,
            Format::Stereo8b22Khz,
            Format::Stereo16b22Khz,
            Format::Mono8b44Khz,
            Format::Mono16b44Khz,
            Format::Stereo8b44Khz,
            Format::Stereo16b44Khz,
            Format::Mono8b96Khz,
            Format::Mono16b96Khz,
            Format::Stereo8b96Khz,
            Format::Stereo16b96Khz,
        ]
        .iter()
        .copied()
        .filter(|f| (self.caps.dwFormats & *f as u32) != 0)
        .collect()
    }

    /// Number specifying whether the device supports mono (1) or stereo (2) output.
    pub fn channels(&self) -> u16 {
        self.caps.wChannels
    }

    /// Optional functionality supported by the device.
    pub fn functionality(&self) -> Vec<Functionality> {
        [
            Functionality::LrVolume,
            Functionality::Pitch,
            Functionality::PlaybackRate,
            Functionality::Sync,
            Functionality::Volume,
            Functionality::SampleAccurate,
        ]
        .iter()
        .copied()
        .filter(|f| (self.caps.dwSupport & *f as u32) != 0)
        .collect()
    }
}

impl fmt::Debug for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Capabilities")
            .field("manufacturer", &self.manufacturer())
            .field("product", &self.product())
            .field("driver_version", &self.driver_version())
            .field("name", &self.name())
            .field("supported_formats", &self.supported_formats())
            .field("channels", &self.channels())
            .field("functionality", &self.functionality())
            .finish()
    }
}

/// Retrieves the capabilities of a given waveform-audio output device.
pub fn get_capabilities(index: u32) -> Result<Capabilities, Error> {
    let mut caps = MaybeUninit::uninit();
    let result = unsafe {
        waveOutGetDevCapsW(
            index as usize,
            caps.as_mut_ptr(),
            mem::size_of::<WAVEOUTCAPSW>() as u32,
        )
    };
    check_multimedia_error(result)?;
    let caps = unsafe { caps.assume_init() };
    Ok(Capabilities { caps })
}

/// Retrieves the number of waveform-audio output devices present in the system.
pub fn count() -> u32 {
    unsafe { waveOutGetNumDevs() }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C, packed)]
    struct MockCaps {
        w_mid: u16,
        w_pid: u16,
        v_driver_version: u16,
        sz_name: [u16; 32],
    }

    struct MockDevice {
        caps: MockCaps,
    }

    impl MockDevice {
        fn new(w_mid: u16, w_pid: u16, v_driver_version: u16, sz_pname: [u16; 32]) -> Self {
            Self {
                caps: MockCaps {
                    w_mid,
                    w_pid,
                    v_driver_version,
                    sz_name: sz_pname,
                },
            }
        }
    }

    impl MockDevice {
        fn manufacturer(&self) -> Manufacturer {
            let mid = self.caps.w_mid;
            Manufacturer::try_from(mid).expect("unknown manufacturer")
        }

        fn product(&self) -> Option<Product> {
            let pid = self.caps.w_pid;

            Product::try_from(pid).ok()
        }

        fn driver_version(&self) -> (u8, u8) {
            let driver_version = self.caps.v_driver_version;

            let major = ((driver_version) >> 8) & 0xff;
            let minor = driver_version & 0xff;
            (major as u8, minor as u8)
        }

        fn name(&self) -> String {
            let unaligned = &raw const self.caps.sz_name;
            let raw = unsafe { std::ptr::read_unaligned(unaligned) };

            let sz_pname_ptr = raw.as_ptr();
            let name = unsafe { U16CString::from_ptr_str(sz_pname_ptr) };
            name.to_string().expect("non-utf8 product name")
        }

        fn supported_formats(&self) -> Vec<Format> {
            [
                Format::Mono8b11Khz,
                Format::Mono16b11Khz,
                Format::Stereo8b11Khz,
                Format::Stereo16b11Khz,
                Format::Mono8b22Khz,
                Format::Mono16b22Khz,
                Format::Stereo8b22Khz,
                Format::Stereo16b22Khz,
                Format::Mono8b44Khz,
                Format::Mono16b44Khz,
                Format::Stereo8b44Khz,
                Format::Stereo16b44Khz,
                Format::Mono8b96Khz,
                Format::Mono16b96Khz,
                Format::Stereo8b96Khz,
                Format::Stereo16b96Khz,
            ]
            .to_vec()
        }
    }

    #[test]
    fn test_manufacturer() {
        let device = MockDevice::new(Manufacturer::Microsoft as u16, 0, 0, [0; 32]);
        assert_eq!(device.manufacturer(), Manufacturer::Microsoft);
    }

    #[test]
    fn test_product() {
        let device = MockDevice::new(0, Product::WaveOut as u16, 0, [0; 32]);
        assert_eq!(device.product(), Some(Product::WaveOut));
    }

    #[test]
    fn test_driver_version() {
        let device = MockDevice::new(0, 0, 0x1307, [0; 32]);
        assert_eq!(device.driver_version(), (19, 7));
    }

    #[test]
    fn test_name() {
        let name = "Test Device";
        let mut name_array = [0; 32];
        for (i, c) in name.encode_utf16().enumerate() {
            name_array[i] = c;
        }
        let device = MockDevice::new(0, 0, 0, name_array);
        assert_eq!(device.name(), name);
    }

    #[test]
    fn test_supported_formats() {
        let device = MockDevice::new(0, 0, 0, [0; 32]);
        let formats = device.supported_formats();
        assert!(formats.contains(&Format::Mono8b11Khz));
        assert!(formats.contains(&Format::Stereo16b96Khz));
    }
}
