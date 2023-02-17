use std::sync::{Arc, Mutex};

use crate::{
	data::{Channel, SystemState},
	resources::strings,
	windows::{Window, WindowName},
};

//=================

#[derive(Default)]
pub struct MixerWindow {}

impl Window for MixerWindow {
	fn show(
		&mut self,
		ctx: &egui::Context,
		name: &WindowName,
		open: &mut bool,
		state: &mut Arc<Mutex<SystemState>>,
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.show(ctx, |ui| self.ui(ui, state));
	}

	fn ui(&mut self, ui: &mut egui::Ui, state: &mut Arc<Mutex<SystemState>>) {
		let state = &mut state.lock().unwrap();
		let mut remove_queue = Vec::with_capacity(1);
		egui::TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			if ui.button(strings::MIXER_NEW_CHANNEL).clicked() {
				state.mixer.add_channel();
			}
		});

		egui::ScrollArea::horizontal().show(ui, |ui| {
			ui.horizontal(|ui| {
				state.mixer
					.channels
					.iter_mut()
					.enumerate()
					.for_each(|(idx, c)| view(ui, c, idx, &mut remove_queue));
			});
		});

		state.mixer.remove_queue = remove_queue;
		state.mixer.clean_channels();
	}

	fn as_any(&mut self) -> &mut dyn std::any::Any {
		self
	}

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut> {
		Some(egui::KeyboardShortcut::new(
			egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
			egui::Key::M,
		))
	}
}

fn view_contents(
	ui: &mut egui::Ui,
	channel: &mut Channel,
	index: usize,
	remove_queue: &mut Vec<usize>,
) {
	ui.add(egui::TextEdit::singleline(&mut channel.name)
		.desired_width(64.0)
		.font(egui::TextStyle::Small)
		.min_size(egui::Vec2::new(64.0, 12.0)));

	// Panning
	ui.add(egui_extras_xt::knobs::AudioKnob::new(&mut channel.panning)
		.range(-1.0..=1.0)
		.spread(0.75)
		.drag_length(4.0)
		.animated(false)
		.shape(egui_extras_xt::common::WidgetShape::Circle));

	let label = if channel.panning == 0.0 {
		""
	} else if channel.panning > 0.0 {
		"R"
	} else {
		"L"
	};

	ui.label(format!("{:.0}% {}", (channel.panning * 100.0).abs(), label));

	// Volume
	ui.add_enabled_ui(!channel.muted, |ui| {
		ui.horizontal(|ui| {
			ui.allocate_space(egui::Vec2::splat(16.0));
			ui.add(egui::Slider::new(&mut channel.volume, -30.0..=6.0)
				.vertical()
				.show_value(false))
				.on_hover_text_at_pointer(format!("{:.1} dB", channel.volume));
		});
	});

	ui.toggle_value(&mut channel.muted, "M");

	// First index is the master channel - let's not remove that!
	if index > 0 {
		if ui.button("❌").clicked() {
			remove_queue.push(index);
		}
	}
}

fn view(ui: &mut egui::Ui, channel: &mut Channel, index: usize, remove_queue: &mut Vec<usize>) {
	ui.group(|ui| {
		ui.allocate_ui_with_layout(
			egui::Vec2::new(64.0, 256.0),
			egui::Layout::top_down_justified(egui::Align::Center),
			|ui| view_contents(ui, channel, index, remove_queue),
		);
	});
}
