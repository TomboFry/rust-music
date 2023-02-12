use crate::{resources::strings, utilities::audio, windows::Window};
use cpal::traits::DeviceTrait;
use cpal::Device;
use egui::ComboBox;

pub struct Settings {
	available_inputs: Vec<Device>,
	available_outputs: Vec<Device>,
	active_input_index: usize,
	active_output_index: usize,
}

impl Default for Settings {
	fn default() -> Self {
		let devices = audio::get_devices();

		let default_input_name = devices.input_default.name().unwrap();
		let default_output_name = devices.output_default.name().unwrap();

		Self {
			active_input_index: Settings::get_device_index(
				&devices.input_list,
				&default_input_name,
			),
			active_output_index: Settings::get_device_index(
				&devices.output_list,
				&default_output_name,
			),
			available_inputs: devices.input_list,
			available_outputs: devices.output_list,
		}
	}
}

impl Settings {
	pub fn get_device_index(devices: &[Device], name: &str) -> usize {
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
}

impl Window for Settings {
	fn show(&mut self, ctx: &egui::Context, name: &'static str, open: &mut bool) {
		egui::Window::new(name)
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
					output_changed = ui
						.selectable_value(
							&mut self.active_output_index,
							index,
							device.name().unwrap(),
						)
						.changed();
				}
			});

		ui.add_space(16.0);

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(self.get_device_input_name())
			.show_ui(ui, |ui| {
				for (index, device) in self.available_inputs.iter().enumerate() {
					input_changed = ui
						.selectable_value(
							&mut self.active_input_index,
							index,
							device.name().unwrap(),
						)
						.changed();
				}
			});
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
