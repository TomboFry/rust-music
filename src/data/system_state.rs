use super::{AudioEngineEvent, AudioSettings};
use rtrb::Producer;

pub struct SystemState {
	pub audio: AudioSettings,
}

impl SystemState {
	pub fn new(audio_update_tx: Producer<AudioEngineEvent>) -> Self {
		Self {
			audio: AudioSettings::new(audio_update_tx),
		}
	}
}
