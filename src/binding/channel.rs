use std::{ collections::HashMap, sync::{ Arc, atomic::{ Ordering, AtomicBool } } };

use tokio::sync::{ RwLock, mpsc::{ channel, Sender, Receiver } };

use songbird::{ Call, input::Input, tracks::TrackHandle };

use super::{ communication::ContextManager };


pub type IsMPSCReleased = Arc<AtomicBool>;
pub type AudioSender = Sender<Vec<u8>>;
pub type AudioReceiver = Arc<RwLock<Receiver<Vec<u8>>>>;
pub type AudioMPSC = (AudioSender, AudioReceiver);
pub type AcquiredAudioMPSC = (AudioMPSC, IsMPSCReleased);


pub struct Id { pub u64: u64, pub i64: i64 }
impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Self { u64: id, i64: id as _ }
    }
}

pub struct TrackData {
    pub handle: TrackHandle,
    pub sender: AudioSender,
    released: IsMPSCReleased,
    pub buffer_id: String
}
impl TrackData {
    pub fn new(
        channel_id: u64, handle: TrackHandle,
        sender: AudioSender, released: IsMPSCReleased
    ) -> Self {
        Self { buffer_id: format!(
            "{}b{}", channel_id, handle.uuid()
        ), handle, sender, released }
    }
}

impl Drop for TrackData {
    fn drop(&mut self) {
        self.released.store(true, Ordering::SeqCst)
    }
}


pub struct Core {
    pub call: Call,
    pub ctx: ContextManager,
    pub tracks: HashMap<u128, TrackData>
}

pub struct Channel {
    pub id: Id, pub(crate) core: Core,
    audio_mpsc: Vec<AcquiredAudioMPSC>
}

impl Channel {
    pub fn new(id: u64, call: Call) -> Self{
        let id_string = id.to_string();
        Self {
            id: Id::from(id), core: Core {
                call: call,
                ctx: ContextManager { key: id_string },
                tracks: HashMap::new()
            }, audio_mpsc: Vec::new()
        }
    }

    pub unsafe fn get_core(&self) -> &Core { &self.core }
    pub unsafe fn get_mut_core(&mut self) -> &mut Core { &mut self.core }

    pub fn acquire(&mut self) -> AcquiredAudioMPSC {
        // もう使われていないチャネルを探し、使われてないのがあればそれを使用する。
        // 使用する際に、次の検索時に検索が効率良くなるように最後へ移動させる。
        let mut temp;
        for i in 0..self.audio_mpsc.len() {
            temp = &self.audio_mpsc[i];
            if temp.1.load(Ordering::SeqCst) {
                temp.1.store(false, Ordering::SeqCst);
                let return_value = temp.clone();
                let last = self.audio_mpsc.len()-1;
                self.audio_mpsc.swap(i, last);
                return return_value;
            };
        };
        let (tx, rx) = channel::<Vec<u8>>(128);
        let value = (
            (tx, Arc::new(RwLock::new(rx))),
            Arc::new(AtomicBool::new(false))
        );
        let return_value = value.clone();
        self.audio_mpsc.push(value);
        return_value
    }

    pub fn play(&mut self, source: Input, aampsc: &AcquiredAudioMPSC) -> u128 {
        let handle = self.core.call.play_source(source);
        let id = handle.uuid().as_u128();
        self.core.tracks.insert(id, TrackData::new(
            self.id.u64, handle, aampsc.0.0.clone(),
            aampsc.1.clone()
        ));
        id
    }
}