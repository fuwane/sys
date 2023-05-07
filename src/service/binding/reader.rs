//! FuwaNe Service Binding - Reader

use std::{ io::{
    Result as IoResult, Error as IoError, ErrorKind, Read, Seek, SeekFrom
} };

use songbird::input::reader::MediaSource;

use crate::MANAGERS;


pub struct WasmAudioReader {
    pub manager_id: u32,
    pub channel_id: u64
}

impl MediaSource for WasmAudioReader {
    fn byte_len(&self) -> Option<u64> { None }
    fn is_seekable(&self) -> bool { false }
}

impl Read for WasmAudioReader {
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<usize> {
        println!("WARNING: Couldn't find manager."); // TODO: Log it.
        Ok(0)
    }
}

impl Seek for WasmAudioReader {
    fn seek(&mut self, _pos: SeekFrom) -> IoResult<u64> {
        // TODO: Support it.
        Err(IoError::new(ErrorKind::Unsupported, "Seeking is not supported yet."))
    }
}
