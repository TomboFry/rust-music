use crate::utilities::audio;
use cpal::traits::DeviceTrait;

pub struct AudioSettings {
	pub available_inputs: Vec<cpal::Device>,
	pub available_outputs: Vec<cpal::Device>,
	pub active_input_index: usize,
	pub active_output_index: usize,
	pub output_sample_rate: u32,
	pub output_config_range: Option<cpal::SupportedStreamConfigRange>,
}

impl Default for AudioSettings {
	fn default() -> Self {
		let devices = audio::get_devices();

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
		let config = self.available_outputs[self.active_output_index]
			.supported_output_configs()
			.unwrap()
			.next()
			.unwrap();

		self.output_sample_rate = self
			.output_sample_rate
			.clamp(config.min_sample_rate().0, config.max_sample_rate().0);

		self.output_config_range = Some(config);
	}
}
