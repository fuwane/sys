use std::{ io::{
    Result as IoResult, Error as IoError, ErrorKind,
    Read, Seek, SeekFrom
} };

use songbird::input::reader::MediaSource;

use super::channel::AudioReceiver;


pub struct WasmAudioReader {
    pub(crate) channel_id: u64,
    pub receiver: AudioReceiver
}

impl MediaSource for WasmAudioReader {
    fn byte_len(&self) -> Option<u64> { None }
    fn is_seekable(&self) -> bool { false }
}

impl Read for WasmAudioReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        Ok(if let Some(frame) = self.receiver.blocking_write().blocking_recv() {
            // TODO: Log it.
            if frame.len() > buf.len() {
                println!("WARNING: Frame size is not valid.");
                0
            } else {
                for (i, value) in frame.iter().enumerate() {
                    buf[i] = *value;
                };
                buf.len()
            }
        } else { 0 })
    }
}

impl Seek for WasmAudioReader {
    fn seek(&mut self, _pos: SeekFrom) -> IoResult<u64> {
        // TODO: Support it.
        Err(IoError::new(ErrorKind::Unsupported, "Seeking is not supported yet."))
    }
}

impl WasmAudioReader {
    pub fn channel_id(&self) -> &u64 { &self.channel_id }
}