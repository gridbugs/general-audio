pub use general_audio::*;
use rodio::Sink;

mod common;

#[derive(Debug)]
pub enum Error {
    FailedToCreateOutputStream,
}

#[cfg(not(any(target_os = "windows", feature = "force_dedicated_audio_thread")))]
mod audio_on_main_thread;
#[cfg(not(any(target_os = "windows", feature = "force_dedicated_audio_thread")))]
use audio_on_main_thread as platform_specific;

#[cfg(any(target_os = "windows", feature = "force_dedicated_audio_thread"))]
mod dedicated_audio_thread;
#[cfg(any(target_os = "windows", feature = "force_dedicated_audio_thread"))]
use dedicated_audio_thread as platform_specific;

pub use platform_specific::NativeAudioPlayer;

#[derive(Clone)]
pub struct NativeSound {
    bytes: &'static [u8],
}

impl NativeSound {
    pub fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

pub struct NativeHandle {
    sink: Sink,
}

impl AudioHandle for NativeHandle {
    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }
    fn volume(&self) -> f32 {
        self.sink.volume()
    }
    fn pause(&self) {
        self.sink.pause();
    }
    fn play(&self) {
        self.sink.play();
    }
    fn background(self) {
        self.sink.detach()
    }
}

impl AudioPlayer for NativeAudioPlayer {
    type Sound = NativeSound;
    type Handle = NativeHandle;
    fn play(&self, sound: &Self::Sound) -> Self::Handle {
        let sink = NativeAudioPlayer::play_bytes(self, sound.bytes);
        NativeHandle { sink }
    }
    fn play_loop(&self, sound: &Self::Sound) -> Self::Handle {
        let sink = NativeAudioPlayer::play_bytes_loop(self, sound.bytes);
        NativeHandle { sink }
    }
    fn load_sound(&self, bytes: &'static [u8]) -> Self::Sound {
        NativeSound::new(bytes)
    }
}
