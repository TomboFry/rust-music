use crate::{resources::strings, windows::Window};
use egui::{ComboBox, Vec2};

pub struct Settings {
	available_inputs: Vec<String>,
	available_outputs: Vec<String>,
	active_input: String,
	active_output: String,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			// TODO: Get list of devices using CPAL and update when necessary
			available_inputs: vec!["Default Input".to_owned(), "Second Input".to_owned()],
			available_outputs: vec!["Default Output".to_owned(), "Second Input".to_owned()],
			active_input: "Default Input".to_owned(),
			active_output: "Default Output".to_owned(),
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
		ComboBox::from_id_source("output-device")
			.width(160.0)
			.selected_text(&self.active_output)
			.show_ui(ui, |ui| {
				self.available_outputs.iter().for_each(|output| {
					ui.selectable_value(&mut self.active_output, output.to_owned(), output);
				})
			});

		ui.allocate_space(Vec2::new(0.0, 16.0));

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		ComboBox::from_id_source("input-device")
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
