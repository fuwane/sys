//! FuwaNe Service Binding - Reader

use std::{ io::{
    Result as IoResult, Error as IoError, ErrorKind, Read, Seek, SeekFrom
} };

use songbird::input::reader::MediaSource;

use super::SINKS;


pub struct WasmAudioReader {
    pub channel_id: u64,
}

impl MediaSource for WasmAudioReader {
    fn byte_len(&self) -> Option<u64> { None }
    fn is_seekable(&self) -> bool { false }
}

impl Read for WasmAudioReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        if let Some(sink) = SINKS.blocking_lock().get_mut(&self.channel_id) {
            if sink.length == buf.len() {
                if let Some(data) = sink.buffer.pop_front() {
                    for (i, v) in data.iter().enumerate() {
                        buf[i] = *v;
                    };
                };
            } else {
                sink.length = buf.len();
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