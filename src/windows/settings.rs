use crate::{resources::strings, utilities::audio, windows::Window};
use cpal::traits::DeviceTrait;
use cpal::{Device, SupportedStreamConfigRange};
use egui::ComboBox;

use super::WindowName;

pub struct Settings {
	pub available_inputs: Vec<Device>,
	pub available_outputs: Vec<Device>,
	pub active_input_index: usize,
	pub active_output_index: usize,
	pub output_sample_rate: u32,
	output_config_range: Option<SupportedStreamConfigRange>,
}

impl Default for Settings {
	fn default() -> Self {
		let devices = audio::get_devices();

		let default_input_name = devices.input_default.name().unwrap();
		let default_output_name = devices.output_default.name().unwrap();
		let active_input_index =
			Settings::get_device_index(&devices.input_list, &default_input_name);
		let active_output_index =
			Settings::get_device_index(&devices.output_list, &default_output_name);

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

impl Settings {
	fn get_device_index(devices: &[Device], name: &str) -> usize {
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

	fn get_device_output_name(&self) -> String {
		self.available_outputs[self.active_output_index]
			.name()
			.unwrap()
	}

	fn get_device_input_name(&self) -> String {
		self.available_inputs[self.active_input_index]
			.name()
			.unwrap()
	}

	fn update_output_config(&mut self) {
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

impl Window for Settings {
	fn show(&mut self, ctx: &egui::Context, name: &WindowName, open: &mut bool) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui));
	}

	fn ui(&mut self, ui: &mut egui::Ui) {
		let mut output_changed = false;
		let mut input_changed = false;

		ui.label(strings::SETTINGS_OUTPUT_DEVICE);
		ComboBox::from_id_source("settings-output-device")
			.width(160.0)
			.selected_text(self.get_device_output_name())
			.show_ui(ui, |ui| {
				for (index, device) in self.available_outputs.iter().enumerate() {
					if ui.selectable_value(
						&mut self.active_output_index,
						index,
						device.name().unwrap(),
					)
					.changed()
					{
						output_changed = true;
					}
				}
			});

		ui.add_space(16.0);

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(self.get_device_input_name())
			.show_ui(ui, |ui| {
				for (index, device) in self.available_inputs.iter().enumerate() {
					if ui.selectable_value(
						&mut self.active_input_index,
						index,
						device.name().unwrap(),
					)
					.changed()
					{
						input_changed = true;
					}
				}
			});

		ui.add_enabled_ui(self.output_config_range.is_some(), |ui| {
			let mut drag_value =
				egui::DragValue::new(&mut self.output_sample_rate).suffix(" Hz");

			if let Some(range) = self.output_config_range.as_ref() {
				let min = range.min_sample_rate().0;
				let max = range.max_sample_rate().0;
				drag_value = drag_value.clamp_range(min..=max);
			}

			ui.add_space(16.0);
			ui.label(strings::SETTINGS_SAMPLE_RATE);
			ui.add(drag_value);
		});

		if output_changed {
			self.update_output_config();
		}
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
