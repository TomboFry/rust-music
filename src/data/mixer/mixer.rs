use super::Channel;
use crate::{
	data::{EngineConfig, Project},
	resources::strings,
};
use std::sync::RwLockReadGuard;

pub struct Mixer {
	pub channels: Vec<Channel>,
}

impl Mixer {
	pub fn add_channel(&mut self) {
		let len = self.channels.len();
		let name = format!("{} {}", strings::CHANNEL_DEFAULT_NAME, len + 1);
		let channel = Channel::new(&name);

		self.channels.push(channel);
	}

	pub fn render_buffer(
		&self,
		project: &RwLockReadGuard<Project>,
		buffer_size: usize,
		info: &cpal::OutputCallbackInfo,
		config: &mut EngineConfig,
	) -> Vec<f64> {
		let mixes = self
			.channels
			.iter()
			.map(|c| c.render_buffer(project, buffer_size, info, config))
			.collect::<Vec<_>>();

		// Mix all channels together
		// TODO: There must be a faster way
		let mut mix = vec![0.0; buffer_size];
		mixes.iter().for_each(|c| {
			c.iter().enumerate().for_each(|(index, sample)| {
				mix[index] = *sample + mix[index];
			})
		});

		mix
	}
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new("Master"),
				Channel::new("Channel 1"),
				Channel::new("Channel 2"),
			],
		}
	}
}
