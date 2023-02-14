use super::{AudioSettings, Mixer, Project, Sampler};

#[derive(Default)]
pub struct SystemState {
	pub project: Project,
	pub mixer: Mixer,
	pub sampler: Sampler,
	pub audio: AudioSettings,
}
