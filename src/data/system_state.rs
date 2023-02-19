use std::collections::VecDeque;

use super::{AudioEngineEvent, AudioSettings};
use crate::resources::UiEvent;
use rtrb::Producer;

pub struct SystemState {
	pub audio: AudioSettings,
	pub ui_events: VecDeque<UiEvent>,
}

impl SystemState {
	pub fn new(audio_update_tx: Producer<AudioEngineEvent>) -> Self {
		Self {
			audio: AudioSettings::new(audio_update_tx),
			ui_events: VecDeque::with_capacity(2),
		}
	}

	pub fn dispatch(&mut self, event: UiEvent) {
		self.ui_events.push_back(event);
	}
}
