//! FuwaNe Service Binding - Reader

use std::{ io::{
    Result as IoResult, Error as IoError, ErrorKind, Read, Seek, SeekFrom
} };

use songbird::input::reader::MediaSource;

use super::AUDIO_BUFFER;


pub struct WasmAudioReader {
    pub channel_id: u64,
}

impl MediaSource for WasmAudioReader {
    fn byte_len(&self) -> Option<u64> { None }
    fn is_seekable(&self) -> bool { false }
}

impl Read for WasmAudioReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        if let Some(deque) = AUDIO_BUFFER.blocking_lock().get_mut(&self.channel_id) {
            if let Some(data) = deque.pop_front() {
                for (i, v) in data.iter().enumerate() {
                    buf[i] = *v;
                };
            };
        };
        Ok(0)
    }
}

impl Seek for WasmAudioReader {
    fn seek(&mut self, _pos: SeekFrom) -> IoResult<u64> {
        // TODO: Support it.
        Err(IoError::new(ErrorKind::Unsupported, "Seeking is not supported yet."))
    }
}