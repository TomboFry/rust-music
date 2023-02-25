use crate::data::{EngineConfig, Project};
use std::sync::RwLockReadGuard;

pub struct Channel {
	pub volume: f64,
	pub panning: f32,
	pub muted: bool,
	pub name: String,
	pub effects: Vec<vst::host::PluginInstance>,
}

impl Channel {
	pub fn new(name: &str) -> Self {
		Self {
			volume: 0.0,
			panning: 0.0,
			muted: false,
			name: name.to_owned(),
			effects: Vec::with_capacity(10),
		}
	}

	pub fn render_buffer(
		&self,
		_project: &RwLockReadGuard<Project>,
		buffer_size: usize,
		_info: &cpal::OutputCallbackInfo,
		config: &EngineConfig,
	) -> Vec<f64> {
		let mut buffer: Vec<f64> = vec![0.0; buffer_size];

		// Save some processing power!
		if self.muted {
			return buffer;
		}

		let mut timer = 0.0;

		// `sample` contains left and right f64 values
		for sample in buffer.chunks_exact_mut(2) {
			let [left, right] = sample else { panic!("Requested 2 channels but got less") };

			let volume = self.volume * 0.01;
			let pan_left = ((self.panning as f64 * -1.0) + 1.0).clamp(0.0, 1.0);
			let pan_right = (self.panning as f64 + 1.0).clamp(0.0, 1.0);

			let val = (rand::random::<f64>() * 2.0) - 1.0;

			timer = (timer + 1.0) % config.sample_rate;

			*left = val * pan_left * volume;
			*right = val * pan_right * volume;
		}

		buffer
	}
}
