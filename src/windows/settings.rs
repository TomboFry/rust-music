use super::{SystemState, Window, WindowName};
use crate::resources::{strings, UiEvent};
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
		state: &Arc<RwLock<SystemState>>,
		ui_events: &mut VecDeque<UiEvent>,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui, state, ui_events));
	}

	fn ui(
		&mut self,
		ui: &mut egui::Ui,
		state: &Arc<RwLock<SystemState>>,
		ui_events: &mut VecDeque<UiEvent>,
	) {
		let state = &mut state.read().unwrap();
		let mut output_changed: Option<usize> = None;
		let mut input_changed: Option<usize> = None;
		let mut sample_rate = state.audio.output_sample_rate;

		ui.label(strings::SETTINGS_OUTPUT_DEVICE);
		egui::ComboBox::from_id_source("settings-output-device")
			.width(160.0)
			.selected_text(state.audio.get_device_output_name())
			.show_ui(ui, |ui| {
				for (index, device) in state.audio.available_outputs.iter().enumerate() {
					ui.selectable_value(&mut output_changed, Some(index), device.name().unwrap());
				}
			});

		ui.add_space(16.0);

		ui.label(strings::SETTINGS_INPUT_DEVICE);
		egui::ComboBox::from_id_source("settings-input-device")
			.width(160.0)
			.selected_text(state.audio.get_device_input_name())
			.show_ui(ui, |ui| {
				for (index, device) in state.audio.available_inputs.iter().enumerate() {
					ui.selectable_value(&mut input_changed, Some(index), device.name().unwrap());
				}
			});

		ui.add_enabled_ui(state.audio.output_config_range.is_some(), |ui| {
			let mut drag_value = egui::DragValue::new(&mut sample_rate).suffix(" Hz");

			if let Some(range) = &state.audio.output_config_range {
				let min = range.min_sample_rate().0;
				let max = range.max_sample_rate().0;
				drag_value = drag_value.clamp_range(min..=max);
			}

			ui.add_space(16.0);
			ui.label(strings::SETTINGS_SAMPLE_RATE);
			ui.add(drag_value);
		});

		if sample_rate != state.audio.output_sample_rate {
			ui_events.push_back(UiEvent::OutputDeviceSampleRate(sample_rate));
		}

		if let Some(output_index) = output_changed {
			ui_events.push_back(UiEvent::OutputDevice(output_index));
		}

		if let Some(input_index) = input_changed {
			ui_events.push_back(UiEvent::InputDevice(input_index));
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
