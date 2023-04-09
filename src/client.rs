//! FuwaNe System - Client

use songbird::Call;


pub struct Client {
    pub call: Call
}


impl Client {
    pub fn new(call: Call) -> Self {
        Self { call: call }
    }
}