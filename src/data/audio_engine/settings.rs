use super::AudioEngineEvent;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use rtrb::Producer;

pub struct AudioSettings {
	pub available_inputs: Vec<cpal::Device>,
	pub available_outputs: Vec<cpal::Device>,
	pub active_input_index: usize,
	pub active_output_index: usize,
	pub output_sample_rate: u32,
	pub output_config_range: Option<cpal::SupportedStreamConfigRange>,
	pub upd_tx: Producer<AudioEngineEvent>,
}

impl AudioSettings {
	pub fn new(upd_tx: Producer<AudioEngineEvent>) -> Result<Self> {
		let host = cpal::default_host();

		let device_output_list = host.output_devices()?.collect::<Vec<_>>();
		let device_input_list = host.input_devices()?.collect::<Vec<_>>();

		let device_output_default = host.default_output_device().unwrap();
		let device_input_default = host.default_input_device().unwrap();

		let default_input_name = device_input_default.name()?;
		let default_output_name = device_output_default.name()?;

		let active_input_index =
			AudioSettings::get_device_index(&device_input_list, &default_input_name);
		let active_output_index =
			AudioSettings::get_device_index(&device_output_list, &default_output_name);

		let mut settings = Self {
			active_input_index,
			active_output_index,
			available_inputs: device_input_list,
			available_outputs: device_output_list,
			output_sample_rate: 48000,
			output_config_range: None,
			upd_tx,
		};

		settings.update_output_config(active_output_index)?;
		// settings.update_input_config();

		Ok(settings)
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

	pub fn update_output_config(&mut self, device_index: usize) -> Result<()> {
		self.active_output_index = device_index;

		let supported_config = self.available_outputs[self.active_output_index]
			.supported_output_configs()?
			.next()
			.unwrap();

		self.output_sample_rate = self.output_sample_rate.clamp(
			supported_config.min_sample_rate().0,
			supported_config.max_sample_rate().0,
		);

		self.output_config_range = Some(supported_config.clone());
		let config = supported_config.with_sample_rate(cpal::SampleRate(self.output_sample_rate));

		self.upd_tx.push(AudioEngineEvent::Disable)?;

		let buffer_size = match config.buffer_size() {
			cpal::SupportedBufferSize::Unknown => 960,
			cpal::SupportedBufferSize::Range { max, min } => 960.clamp(*min, *max),
		};

		self.upd_tx.push(AudioEngineEvent::Enable {
			device_index: self.active_output_index,
			config,
			buffer_size,
		})?;

		Ok(())
	}
}
