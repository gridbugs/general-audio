pub use general_audio::*;

pub struct WebAudioPlayer {
    mime: String,
}

impl WebAudioPlayer {
    pub fn new_with_mime(mime: &str) -> Self {
        Self {
            mime: mime.to_string(),
        }
    }
    pub fn load_sound(&self, bytes: &'static [u8]) -> WebSound {
        let base64_data = base64::encode(bytes);
        let uri = format!("data:{};base64,{}", self.mime, base64_data);
        WebSound { uri }
    }

    pub fn play(&self, sound: &WebSound) -> web_sys::HtmlAudioElement {
        let element = web_sys::HtmlAudioElement::new_with_src(sound.uri.as_str()).unwrap();
        let _ = element.play().unwrap();
        element
    }
}

pub struct WebSound {
    uri: String,
}

pub struct WebHandle {
    element: web_sys::HtmlAudioElement,
    background: bool,
}

impl AudioHandle for WebHandle {
    fn set_volume(&self, volume: f32) {
        self.element.set_volume(volume as f64);
    }
    fn volume(&self) -> f32 {
        self.element.volume() as f32
    }
    fn pause(&self) {
        let _ = self.element.pause();
    }
    fn play(&self) {
        let _ = self.element.play();
    }
    fn background(mut self) {
        self.background = true;
    }
}

impl Drop for WebHandle {
    fn drop(&mut self) {
        if !self.background {
            let _ = self.element.pause();
            let element: &web_sys::HtmlElement = self.element.as_ref();
            element.remove();
        }
    }
}

impl AudioPlayer for WebAudioPlayer {
    type Sound = WebSound;
    type Handle = WebHandle;
    fn play(&self, sound: &Self::Sound) -> Self::Handle {
        let element = WebAudioPlayer::play(self, sound);
        WebHandle {
            element,
            background: false,
        }
    }
    fn play_loop(&self, sound: &Self::Sound) -> Self::Handle {
        let element = WebAudioPlayer::play(self, sound);
        element.set_loop(true);
        WebHandle {
            element,
            background: false,
        }
    }
    fn load_sound(&self, bytes: &'static [u8]) -> Self::Sound {
        WebAudioPlayer::load_sound(self, bytes)
    }
}
