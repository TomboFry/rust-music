use super::WindowName;
use crate::{
	data::{AudioFile, SystemState},
	resources::{strings, PlayState, UiEvent},
	windows::Window,
};
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
	time::Instant,
};

#[derive(Default)]
pub struct SamplerWindow {}

impl Window for SamplerWindow {
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
		let channel_len = state.mixer.channels.len() - 1;

		if ui.button(strings::SAMPLER_ADD_LABEL).clicked() {
			// TODO: Load samples asynchronously
			// ui_events.push_back(UiEvent::AddSample { path: () });
		}

		egui::ScrollArea::vertical().show(ui, |ui| {
			state
				.sampler
				.files
				.iter()
				.enumerate()
				.for_each(|(index, file)| {
					ui.horizontal(|ui| ui_sample(file, index, channel_len, ui, ui_events));
				});
		});
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut> {
		Some(egui::KeyboardShortcut::new(
			egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
			egui::Key::A,
		))
	}
}

fn ui_sample(
	file: &AudioFile,
	index: usize,
	channel_len: usize,
	ui: &mut egui::Ui,
	ui_events: &mut VecDeque<UiEvent>,
) {
	let file_name = file.path.file_name().unwrap().to_str().unwrap();
	let full_path = file.path.as_os_str().to_str().unwrap();

	let mut file_channel = file.channel;
	let mut file_play_state = file.play_state;

	ui.add(egui::DragValue::new(&mut file_channel).clamp_range(0..=channel_len));

	let is_playing = match file.play_state {
		PlayState::Stopped => false,
		PlayState::Playing { .. } => true,
		_ => false,
	};

	let label = if is_playing { "⏹" } else { "▶" };
	if ui.button(label).clicked() {
		if is_playing {
			file_play_state = PlayState::Stopped;
		} else {
			file_play_state = PlayState::Playing {
				start_time: Instant::now(),
			};
		}
	}

	if ui.button("❌").clicked() {
		ui_events.push_back(UiEvent::RemoveSample {
			sample_index: index,
		});
	}

	ui.label(file_name).on_hover_text(full_path);

	if file_channel != file.channel {
		ui_events.push_back(UiEvent::SampleChannel {
			sample_index: index,
			channel_index: file_channel,
		});
	}

	if file_play_state != file.play_state {
		ui_events.push_back(UiEvent::PlayPauseSample {
			sample_index: index,
			play_state: file_play_state,
		});
	}
}
