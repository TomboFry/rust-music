use super::{Mixer, Sampler};
use crate::resources::{strings, PlayState};

pub struct Project {
	pub name: String,
	pub tempo: f64,
	pub time_signature_numerator: usize,
	pub time_signature_denominator: usize,
	pub play_state: PlayState,
	pub mixer: Mixer,
	pub sampler: Sampler,
}

impl Project {
	pub fn new(name: &str, tempo: f64) -> Self {
		Self {
			name: name.to_owned(),
			tempo,
			time_signature_numerator: 4,
			time_signature_denominator: 4,
			play_state: PlayState::Stopped,
			mixer: Mixer::default(),
			sampler: Sampler::default(),
		}
	}
}

impl Default for Project {
	fn default() -> Self {
		Self::new(strings::PROJECT_DEFAULT_NAME, 140.0)
	}
}
