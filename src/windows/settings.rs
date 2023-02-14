use super::{SystemState, Window, WindowName};
use crate::resources::strings;
use cpal::traits::DeviceTrait;

pub struct SettingsWindow {}

impl Default for SettingsWindow {
	fn default() -> Self {
		Self {}
	}
}

impl Window for SettingsWindow {
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &mut SystemState,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui, state));
	}

	fn ui(&mut self, ui: &mut egui::Ui, state: &mut SystemState) {
		let mut output_changed = false;
		let mut input_changed = false;

		ui.label(strings::SETTINGS_OUTPUT_DEVICE);
		egui::ComboBox::from_id_source("settings-output-device")
			.width(160.0)
			.selected_text(state.audio_settings.get_device_output_name())
			.show_ui(ui, |ui| {
				for (index, device) in
					state.audio_settings.available_outputs.iter().enumerate()
				{
					if ui.selectable_value(
						&mut state.audio_settings.active_output_index,
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
		egui::ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(state.audio_settings.get_device_input_name())
			.show_ui(ui, |ui| {
				for (index, device) in
					state.audio_settings.available_inputs.iter().enumerate()
				{
					if ui.selectable_value(
						&mut state.audio_settings.active_input_index,
						index,
						device.name().unwrap(),
					)
					.changed()
					{
						input_changed = true;
					}
				}
			});

		ui.add_enabled_ui(state.audio_settings.output_config_range.is_some(), |ui| {
			let mut drag_value =
				egui::DragValue::new(&mut state.audio_settings.output_sample_rate)
					.suffix(" Hz");

			if let Some(range) = state.audio_settings.output_config_range.as_ref() {
				let min = range.min_sample_rate().0;
				let max = range.max_sample_rate().0;
				drag_value = drag_value.clamp_range(min..=max);
			}

			ui.add_space(16.0);
			ui.label(strings::SETTINGS_SAMPLE_RATE);
			ui.add(drag_value);
		});

		if output_changed {
			state.audio_settings.update_output_config();
		}
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut> {
		Some(egui::KeyboardShortcut::new(
			egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
			egui::Key::S,
		))
	}
}
