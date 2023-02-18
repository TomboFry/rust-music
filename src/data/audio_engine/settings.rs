use super::AudioEngineEvent;
use cpal::traits::{DeviceTrait, HostTrait};
use rtrb::Producer;

pub struct DeviceResult {
	pub input_default: cpal::Device,
	pub output_default: cpal::Device,
	pub input_list: Vec<cpal::Device>,
	pub output_list: Vec<cpal::Device>,
}

impl DeviceResult {
	pub fn get_devices() -> DeviceResult {
		let host = cpal::default_host();
		DeviceResult {
			input_default: host.default_input_device().unwrap(),
			input_list: host.input_devices().unwrap().collect(),

			output_default: host.default_output_device().unwrap(),
			output_list: host.output_devices().unwrap().collect(),
		}
	}
}

pub struct AudioSettings {
	pub available_inputs: Vec<cpal::Device>,
	pub available_outputs: Vec<cpal::Device>,
	pub active_input_index: usize,
	pub active_output_index: usize,
	pub output_sample_rate: u32,
	pub output_channels: u32,
	pub output_config_range: Option<cpal::SupportedStreamConfigRange>,
	pub upd_tx: Producer<AudioEngineEvent>,
}

impl AudioSettings {
	pub fn new(upd_tx: Producer<AudioEngineEvent>) -> Self {
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
			output_channels: 2,
			output_config_range: None,
			upd_tx,
		};

		settings.update_output_config(active_output_index);
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

	pub fn update_output_config(&mut self, device_index: usize) {
		self.active_output_index = device_index;

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
			.with_sample_rate(cpal::SampleRate(self.output_sample_rate));

		self.upd_tx
			.push(AudioEngineEvent::Disable)
			.expect("Queue full!");

		let buffer_size = match final_config.buffer_size() {
			cpal::SupportedBufferSize::Unknown => 960,
			cpal::SupportedBufferSize::Range { .. } => 960,
		};

		self.upd_tx
			.push(AudioEngineEvent::Enable {
				device_index: self.active_output_index,
				channels: self.output_channels,
				config: final_config,
				buffer_size,
			})
			.expect("Queue full!");
	}
}
