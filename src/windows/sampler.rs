use super::WindowName;
use crate::{
	data::{AudioFile, PlayState, SystemState},
	resources::strings,
	windows::Window,
};
use std::{
	sync::{Arc, Mutex},
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
		state: &mut Arc<Mutex<SystemState>>,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.collapsible(false)
			.min_width(380.0)
			.show(ctx, |ui| self.ui(ui, state));
	}

	fn ui(&mut self, ui: &mut egui::Ui, state: &mut Arc<Mutex<SystemState>>) {
		let state = &mut state.lock().unwrap();
		let channel_len = state.mixer.channels.len() - 1;
		let mut remove_queue = Vec::with_capacity(1);

		if ui.button(strings::SAMPLER_ADD_LABEL).clicked() {
			state.sampler.add_samples();
		}

		egui::ScrollArea::vertical().show(ui, |ui| {
			state.sampler
				.files
				.iter_mut()
				.enumerate()
				.for_each(|(index, file)| {
					ui.horizontal(|ui| {
						ui_sample(
							file,
							index,
							channel_len,
							ui,
							&mut remove_queue,
						)
					});
				});
		});

		state.sampler.remove_queue = remove_queue;
		state.sampler.clean_samples();
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
	file: &mut AudioFile,
	index: usize,
	channel_len: usize,
	ui: &mut egui::Ui,
	remove_queue: &mut Vec<usize>,
) {
	let file_name = file.path.file_name().unwrap().to_str().unwrap();
	let full_path = file.path.as_os_str().to_str().unwrap();

	ui.add(egui::DragValue::new(&mut file.channel).clamp_range(0..=channel_len));

	let is_playing = match file.play_state {
		PlayState::Stopped => false,
		PlayState::Playing(_) => true,
		// _ => false,
	};

	let label = if is_playing { "⏹" } else { "▶" };
	if ui.button(label).clicked() {
		if is_playing {
			file.play_state = PlayState::Stopped;
		} else {
			file.play_state = PlayState::Playing(Instant::now());
		}
	}

	if ui.button("❌").clicked() {
		remove_queue.push(index);
	}

	ui.label(file_name).on_hover_text(full_path);
}
