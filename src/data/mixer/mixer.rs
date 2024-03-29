use super::Channel;
use crate::{
	data::{EngineConfig, Project},
	resources::strings,
};
use std::sync::RwLockReadGuard;

pub struct Mixer {
	pub channels: Vec<Channel>,
	pub selected_channel: Option<usize>,
}

impl Default for Mixer {
	fn default() -> Self {
		Self {
			channels: vec![
				Channel::new("Master"),
				Channel::new("Channel 1"),
				Channel::new("Channel 2"),
			],
			selected_channel: None,
		}
	}
}

impl Mixer {
	pub fn add_channel(&mut self) {
		let len = self.channels.len();
		let name = format!("{} {}", strings::CHANNEL_DEFAULT_NAME, len + 1);
		let channel = Channel::new(&name);

		self.channels.push(channel);
	}

	pub fn render_buffer<T: cpal::Sample + cpal::FromSample<f64> + std::ops::Add<Output = T>>(
		&self,
		data: &mut [T],
		project: &RwLockReadGuard<Project>,
		info: &cpal::OutputCallbackInfo,
		config: &mut EngineConfig,
	) {
		let mixes = self
			.channels
			.iter()
			.map(|c| c.render_buffer(project, data.len(), info, config))
			.collect::<Vec<_>>();

		// Clear buffer first
		data.fill(T::from_sample(0.0));

		// Mix all channels together
		// let mut mix = vec![0.0; buffer_size];
		mixes.iter().for_each(|c| {
			c.iter().enumerate().for_each(|(index, sample)| {
				data[index] = T::from_sample(*sample) + data[index];
			});
		});
	}
}
