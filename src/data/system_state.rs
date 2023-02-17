use super::{AudioEngineEvent, AudioSettings, Mixer, Project, Sampler};
use rtrb::Producer;

pub struct SystemState {
	pub project: Project,
	pub mixer: Mixer,
	pub sampler: Sampler,
	pub audio: AudioSettings,
}

impl SystemState {
	pub fn new(audio_update_tx: Producer<AudioEngineEvent>) -> Self {
		Self {
			project: Project::default(),
			mixer: Mixer::default(),
			sampler: Sampler::default(),
			audio: AudioSettings::new(audio_update_tx),
		}
	}
}
