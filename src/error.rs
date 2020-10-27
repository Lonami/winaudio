use winapi::um::mmsystem::*;

enum_with_try_from!(
/// Errors that can occur when working with Windows' sound API.
pub enum Error(u32) {
    /// Unspecified error.
    Error = MMSYSERR_ERROR,
    /// Device ID out of range.
    BadDeviceId = MMSYSERR_BADDEVICEID,
    /// Driver failed enable.
    NotEnabled = MMSYSERR_NOTENABLED,
    /// Device already allocated.
    Allocated = MMSYSERR_ALLOCATED,
    /// Device handle is invalid.
    InvalidHandle = MMSYSERR_INVALHANDLE,
    /// No device driver present.
    NoDriver = MMSYSERR_NODRIVER,
    /// Memory allocation error.
    NoMemory = MMSYSERR_NOMEM,
    /// Function isn't supported.
    NotSupported = MMSYSERR_NOTSUPPORTED,
    /// Error value out of range.
    BadErrorNumber = MMSYSERR_BADERRNUM,
    /// Invalid flag passed.
    InvalidFlag = MMSYSERR_INVALFLAG,
    /// Invalid parameter passed.
    InvalidParam = MMSYSERR_INVALPARAM,
    /// Handle being used.
    HandleBusy = MMSYSERR_HANDLEBUSY,
    /// Specified alias not found.
    InvalidAlias = MMSYSERR_INVALIDALIAS,
    /// Bad registry database.
    BadDatabase = MMSYSERR_BADDB,
    /// Registry key not found.
    KeyNotFound = MMSYSERR_KEYNOTFOUND,
    /// Registry read error.
    ReadError = MMSYSERR_READERROR,
    /// Registry write error.
    WriteError = MMSYSERR_WRITEERROR,
    /// Registry delete error.
    DeleteError = MMSYSERR_DELETEERROR,
    /// Registry value not found.
    ValueNotFound = MMSYSERR_VALNOTFOUND,
    /// Driver does not call DriverCallback.
    NoDriverCallback = MMSYSERR_NODRIVERCB,
    /// More data to be returned.
    MoreData = MMSYSERR_MOREDATA,
    /// Attempted to open with an unsupported waveform-audio format.
    BadFormat = WAVERR_BADFORMAT,
    /// There are still buffers in the queue.
    StillPlaying = WAVERR_STILLPLAYING,
    /// The data block pointed to by the parameter hasn't been prepared.
    Unprepared = WAVERR_UNPREPARED,
    /// The device is synchronous but the device was opened without using the `AllowSync` flag.
    Sync = WAVERR_SYNC,
});
