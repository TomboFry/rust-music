use super::{Mixer, Project, Sampler};

pub struct SystemState {
	pub project: Project,
	pub mixer: Mixer,
	pub sampler: Sampler,
}

impl Default for SystemState {
	fn default() -> Self {
		Self {
			project: Project::default(),
			mixer: Mixer::default(),
			sampler: Sampler::default(),
		}
	}
}
