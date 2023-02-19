use super::{Project, Window, WindowName};
use crate::{
	data::SystemState,
	resources::{strings, UiEvent},
};
use cpal::traits::DeviceTrait;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct SettingsWindow {}

impl Window for SettingsWindow {
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &Arc<RwLock<Project>>,
		system: &mut SystemState,
		ui_events: &mut VecDeque<UiEvent>,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui, state, system, ui_events));
	}

	fn ui(
		&mut self,
		ui: &mut egui::Ui,
		_state: &Arc<RwLock<Project>>,
		system: &mut SystemState,
		_ui_events: &mut VecDeque<UiEvent>,
	) {
		let mut output_index = system.audio.active_output_index;
		let mut input_changed: Option<usize> = None;
		let mut sample_rate = system.audio.output_sample_rate;

		ui.label(strings::SETTINGS_OUTPUT_DEVICE);
		egui::ComboBox::from_id_source("settings-output-device")
			.width(160.0)
			.selected_text(system.audio.get_device_output_name())
			.show_ui(ui, |ui| {
				for (index, device) in system.audio.available_outputs.iter().enumerate() {
					ui.selectable_value(&mut output_index, index, device.name().unwrap());
				}
			});

		ui.add_space(16.0);

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		ui.label("To be implemented.");
		egui::ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(system.audio.get_device_input_name())
			.show_ui(ui, |ui| {
				for (index, device) in system.audio.available_inputs.iter().enumerate() {
					ui.selectable_value(&mut input_changed, Some(index), device.name().unwrap());
				}
			});

		ui.add_enabled_ui(system.audio.output_config_range.is_some(), |ui| {
			let mut drag_value = egui::DragValue::new(&mut sample_rate).suffix(" Hz");

			if let Some(range) = &system.audio.output_config_range {
				let min = range.min_sample_rate().0;
				let max = range.max_sample_rate().0;
				drag_value = drag_value.clamp_range(min..=max);
			}

			ui.add_space(16.0);
			ui.label(strings::SETTINGS_SAMPLE_RATE);
			ui.add(drag_value);
		});

		if sample_rate != system.audio.output_sample_rate {
			system.audio.update_output_config(output_index);
		}

		if output_index != system.audio.active_output_index {
			system.audio.update_output_config(output_index);
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
