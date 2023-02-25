use super::{AudioEngineEvent, AudioSettings, VstSettings};
use crate::data::UiEvent;
use rtrb::Producer;
use std::collections::VecDeque;

pub struct SystemState {
	pub audio: AudioSettings,
	pub vsts: VstSettings,
	pub ui_events: VecDeque<UiEvent>,
}

impl SystemState {
	pub fn new(audio_update_tx: Producer<AudioEngineEvent>) -> Self {
		Self {
			audio: AudioSettings::new(audio_update_tx).unwrap(),
			vsts: VstSettings::default(),
			ui_events: VecDeque::with_capacity(2),
		}
	}

	pub fn dispatch(&mut self, event: UiEvent) {
		self.ui_events.push_back(event);
	}
}
