use crate::{resources::strings, utilities::audio, windows::Window};
use cpal::traits::DeviceTrait;
use egui::ComboBox;

pub struct Settings {
	available_inputs: Vec<String>,
	available_outputs: Vec<String>,
	active_input: String,
	active_output: String,
}

impl Default for Settings {
	fn default() -> Self {
		let devices = audio::get_devices();
		Self {
			available_inputs: devices
				.input_list
				.iter()
				.map(|device| device.name().unwrap())
				.collect(),

			available_outputs: devices
				.output_list
				.iter()
				.map(|device| device.name().unwrap())
				.collect(),

			active_input: devices.input_default.name().unwrap(),
			active_output: devices.output_default.name().unwrap(),
		}
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
		ui.label(strings::SETTINGS_OUTPUT_DEVICE);
		ComboBox::from_id_source("settings-output-device")
			.width(160.0)
			.selected_text(&self.active_output)
			.show_ui(ui, |ui| {
				self.available_outputs.iter().for_each(|output| {
					ui.selectable_value(&mut self.active_output, output.to_owned(), output);
				})
			});

		ui.add_space(16.0);

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(&self.active_input)
			.show_ui(ui, |ui| {
				self.available_inputs.iter().for_each(|input| {
					ui.selectable_value(&mut self.active_input, input.to_owned(), input);
				})
			});
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
