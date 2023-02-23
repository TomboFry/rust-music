use crate::{
	data::{EngineConfig, Project},
	utilities::format::duration_since_play_state,
};
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
		project: &RwLockReadGuard<Project>,
		buffer_size: usize,
		_info: &cpal::OutputCallbackInfo,
		config: &EngineConfig,
	) -> Vec<f64> {
		let mut buffer: Vec<f64> = vec![0.0; buffer_size];

		// Theoretically, the milliseconds since playing started.
		// Realistically, there is random fluctuation in time
		let mut timer = duration_since_play_state(&project.play_state, Some(config.buffer_start))
			.as_nanos() as f64
			/ 1_000_000.0;

		// Save some processing power!
		if self.muted {
			return buffer;
		}

		// `sample` contains left and right f64 values
		for sample in buffer.chunks_exact_mut(2) {
			let [left, right] = sample else { panic!("Requested 2 channels but got less") };

			let volume = self.volume * 0.01;
			let pan_left = ((self.panning as f64 * -1.0) + 1.0).clamp(0.0, 1.0);
			let pan_right = (self.panning as f64 + 1.0).clamp(0.0, 1.0);

			let val = rand::random::<f64>();

			timer = (timer + 1.0) % config.sample_rate;

			*left = val * pan_left * volume;
			*right = val * pan_right * volume;
		}

		buffer
	}
}
