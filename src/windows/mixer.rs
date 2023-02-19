use crate::{
	data::{Channel, Project, SystemState},
	resources::{strings, UiEvent},
	windows::{Window, WindowName},
};
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct MixerWindow {}

impl Window for MixerWindow {
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
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.show(ctx, |ui| self.ui(ui, state, system, ui_events));
	}

	fn ui(
		&mut self,
		ui: &mut egui::Ui,
		state: &Arc<RwLock<Project>>,
		_system: &mut SystemState,
		ui_events: &mut VecDeque<UiEvent>,
	) {
		let state = state.read().unwrap();
		egui::TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			if ui.button(strings::MIXER_NEW_CHANNEL).clicked() {
				ui_events.push_back(UiEvent::AddChannel);
			}
		});

		egui::ScrollArea::horizontal().show(ui, |ui| {
			ui.horizontal(|ui| {
				state
					.mixer
					.channels
					.iter()
					.enumerate()
					.for_each(|(idx, c)| view(ui, c, idx, ui_events));
			});
		});
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
	channel: &Channel,
	index: usize,
	ui_events: &mut VecDeque<UiEvent>,
) {
	let mut channel_name = channel.name.clone();
	let mut channel_panning = channel.panning;
	let mut channel_volume = channel.volume;
	let mut channel_muted = channel.muted;

	ui.add(
		egui::TextEdit::singleline(&mut channel_name)
			.desired_width(64.0)
			.font(egui::TextStyle::Small)
			.min_size(egui::Vec2::new(64.0, 12.0)),
	);

	// Panning
	ui.add(
		egui_extras_xt::knobs::AudioKnob::new(&mut channel_panning)
			.range(-1.0..=1.0)
			.spread(0.75)
			.drag_length(4.0)
			.animated(false)
			.shape(egui_extras_xt::common::WidgetShape::Circle),
	);

	let label = if channel_panning == 0.0 {
		""
	} else if channel_panning > 0.0 {
		"R"
	} else {
		"L"
	};

	ui.label(format!("{:.0}% {}", (channel_panning * 100.0).abs(), label));

	// Volume
	ui.add_enabled_ui(!channel_muted, |ui| {
		ui.horizontal(|ui| {
			ui.allocate_space(egui::Vec2::splat(16.0));
			ui.add(
				egui::Slider::new(&mut channel_volume, 0.0..=100.0)
					.smart_aim(false)
					.vertical()
					.show_value(false),
			)
			.on_hover_text_at_pointer(format!("{:.1} dB", channel_volume));
		});
	});

	ui.toggle_value(&mut channel_muted, "M");

	// First index is the master channel - let's not remove that!
	if index > 0 {
		if ui.button("❌").clicked() {
			ui_events.push_back(UiEvent::RemoveChannel {
				channel_index: index,
			});
		}
	}

	if channel_name != channel.name {
		ui_events.push_back(UiEvent::ChannelName {
			channel_index: index,
			name: channel_name,
		});
	}

	if channel_panning != channel.panning {
		ui_events.push_back(UiEvent::ChannelPanning {
			channel_index: index,
			panning: channel_panning,
		});
	}

	if channel_volume != channel.volume {
		ui_events.push_back(UiEvent::ChannelVolume {
			channel_index: index,
			volume: channel_volume,
		});
	}

	if channel_muted != channel.muted {
		ui_events.push_back(UiEvent::ChannelMuted {
			channel_index: index,
			muted: channel_muted,
		});
	}
}

fn view(ui: &mut egui::Ui, channel: &Channel, index: usize, ui_events: &mut VecDeque<UiEvent>) {
	ui.group(|ui| {
		ui.allocate_ui_with_layout(
			egui::Vec2::new(64.0, 256.0),
			egui::Layout::top_down_justified(egui::Align::Center),
			|ui| view_contents(ui, channel, index, ui_events),
		);
	});
}
