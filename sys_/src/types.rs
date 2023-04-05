//! FuwaNe System - Types

use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
enum OpCode {
    Event = 0,
    PluginEvent = 1,
    Resume = 2
}


#[derive(Serialize, Deserialize)]
struct Data {
    
}