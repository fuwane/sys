//! FuwaNe Service Binding - Reader

use std::{ collections::BTreeMap, sync::Arc, io::{
    Result as IoResult, Error as IoError, ErrorKind, Read, Seek, SeekFrom
} };

use songbird::input::reader::MediaSource;


pub struct WasmAudioReader {
    vars: BTreeMap<String, Vec<u8>>
}

impl MediaSource for WasmAudioReader {
    fn byte_len(&self) -> Option<u64> { None }
    fn is_seekable(&self) -> bool { false }
}

impl Read for WasmAudioReader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        Ok(1)
    }
}

impl Seek for WasmAudioReader {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        // TODO: Support it.
        Err(IoError::new(ErrorKind::Unsupported, "Seeking is not supported yet."))
    }
}