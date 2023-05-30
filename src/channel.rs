use std::{ sync::Arc, collections::{ HashMap, VecDeque } };

use songbird::{ Call, input::Input, tracks::TrackHandle };

use crate::binding::{ Sink, communication::ContextManager };


pub struct Id { pub u64: u64, pub i64: i64 }
impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Self { u64: id, i64: id as _ }
    }
}

pub struct TrackData {
    pub handle: TrackHandle,
    pub sink: Sink,
    pub buffer_id: String
}
impl TrackData {
    pub fn new(channel_id: &str, handle: TrackHandle) -> Self {
        Self { handle, sink: Arc::new(VecDeque::new()), buffer_id:  }
    }
}


pub struct Core {
    pub call: Call,
    pub ctx: ContextManager,
    pub tracks: HashMap<u128, TrackData>
}


pub struct Channel {
    pub id: Id, pub(crate) core: Core
}

impl Channel {
    pub fn new(id: u64, call: Call) -> Self{
        let id_string = id.to_string();
        Self {
            id: Id::from(id), core: Core {
                call: call,
                ctx: ContextManager { key: id_string },
                tracks: HashMap::new()
            }
        }
    }

    pub unsafe fn get_core(&self) -> &Core { &self.core }
    pub unsafe fn get_mut_core(&mut self) -> &mut Core { &mut self.core }

    pub fn play(&mut self, source: Input, sink: Sink) -> u128 {
        let handle = self.core.call.play_source(source);
        let id = handle.uuid().as_u128();
        self.core.tracks.insert(id, TrackData { handle, sink });
        id
    }
}