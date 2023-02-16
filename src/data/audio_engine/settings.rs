use super::AudioEngine;
use crate::utilities::audio::DeviceResult;
use cpal::traits::{DeviceTrait, StreamTrait};

pub struct AudioSettings {
	pub available_inputs: Vec<cpal::Device>,
	pub available_outputs: Vec<cpal::Device>,
	pub active_input_index: usize,
	pub active_output_index: usize,
	pub output_sample_rate: u32,
	pub output_config_range: Option<cpal::SupportedStreamConfigRange>,
	pub stream: Option<cpal::Stream>,
}

impl Default for AudioSettings {
	fn default() -> Self {
		let devices = DeviceResult::get_devices();

		let default_input_name = devices.input_default.name().unwrap();
		let default_output_name = devices.output_default.name().unwrap();
		let active_input_index =
			AudioSettings::get_device_index(&devices.input_list, &default_input_name);
		let active_output_index =
			AudioSettings::get_device_index(&devices.output_list, &default_output_name);

		let mut settings = Self {
			active_input_index,
			active_output_index,
			available_inputs: devices.input_list,
			available_outputs: devices.output_list,
			output_sample_rate: 48000,
			output_config_range: None,
			stream: None,
		};

		settings.update_output_config();
		// settings.update_input_config();

		settings
	}
}

impl AudioSettings {
	fn get_device_index(devices: &[cpal::Device], name: &str) -> usize {
		if let Some((idx, _)) = devices
			.iter()
			.enumerate()
			.find(|(_, device)| device.name().unwrap() == name)
		{
			idx
		} else {
			0
		}
	}

	pub fn get_device_output_name(&self) -> String {
		self.available_outputs[self.active_output_index]
			.name()
			.unwrap()
	}

	pub fn get_device_input_name(&self) -> String {
		self.available_inputs[self.active_input_index]
			.name()
			.unwrap()
	}

	pub fn update_output_config(&mut self) {
		// TODO: Better error handling
		let config = self.available_outputs[self.active_output_index]
			.supported_output_configs()
			.unwrap()
			.next()
			.unwrap();

		self.output_sample_rate = self
			.output_sample_rate
			.clamp(config.min_sample_rate().0, config.max_sample_rate().0);

		self.output_config_range = Some(config);

		let final_config = self
			.output_config_range
			.clone()
			.unwrap()
			.with_sample_rate(cpal::SampleRate(self.output_sample_rate))
			.config();

		let (_, mut consumer) = AudioEngine::new();

		// TODO: Move somewhere more appropriate
		self.stream = self.available_outputs[self.active_output_index]
			.build_output_stream(
				&final_config,
				move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
					// TODO: I'll figure out the proper way to do this!
					let chunk = consumer.read_chunk(4096).unwrap();
					let c = chunk.as_slices();
					let d = [c.0, c.1].concat();
					let mut index = 0;
					assert!(d.len() >= data.len());
					chunk.commit(data.len());

					for sample in data {
						*sample = d[index];
						index += 1;
						// *sample = (rand::random::<f32>() * 0.5) - 0.25;
					}
				},
				move |err| {
					eprintln!("{err}");
				},
				None,
			)
			.ok();

		if let Some(stream) = &self.stream {
			stream.play().unwrap();
			println!("Playing!");
		}
	}
}
