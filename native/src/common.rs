use rodio::{Decoder, Sink, Source};
use std::io::Cursor;

pub fn output_stream_handle() -> Option<(rodio::OutputStream, rodio::OutputStreamHandle)> {
    if let Ok((output_stream, output_stream_handle)) = rodio::OutputStream::try_default() {
        Some((output_stream, output_stream_handle))
    } else {
        log::warn!("Unable to find audio device. Audio will be disabled.");
        None
    }
}

pub fn play_bytes(
    output_stream_handle: &rodio::OutputStreamHandle,
    bytes: &'static [u8],
) -> Sink {
    let sink = Sink::try_new(output_stream_handle).unwrap();
    let source = Decoder::new(Cursor::new(bytes)).unwrap();
    sink.append(source);
    sink
}

pub fn play_bytes_loop(
    output_stream_handle: &rodio::OutputStreamHandle,
    bytes: &'static [u8],
) -> Sink {
    let sink = Sink::try_new(output_stream_handle).unwrap();
    let source = Decoder::new(Cursor::new(bytes)).unwrap().repeat_infinite();
    sink.append(source);
    sink
}
