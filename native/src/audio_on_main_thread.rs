use crate::{output_stream_handle, play_bytes, play_bytes_loop, Error};
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct NativeAudioPlayer {
    _output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
}

impl NativeAudioPlayer {
    pub fn try_new_default_device() -> Result<Self, Error> {
        let (output_stream, output_stream_handle) =
            output_stream_handle().ok_or(Error::FailedToCreateOutputStream)?;
        Ok(Self {
            _output_stream: output_stream,
            output_stream_handle,
        })
    }

    pub fn new_default_device() -> Self {
        Self::try_new_default_device().unwrap()
    }

    pub fn play_bytes(&self, bytes: &'static [u8]) -> Sink {
        play_bytes(&self.output_stream_handle, bytes)
    }

    pub fn play_bytes_loop(&self, bytes: &'static [u8]) -> Sink {
        play_bytes_loop(&self.output_stream_handle, bytes)
    }
}
