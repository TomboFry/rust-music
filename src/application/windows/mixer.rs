use super::{Window, WindowName};
use crate::{
	data::{Channel, Project, SystemState, UiEvent},
	resources::strings,
};
use std::sync::{Arc, RwLock};
use vst::prelude::*;

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
	) {
		egui::Window::new(name.as_ref())
			.open(open)
			.resizable(true)
			.collapsible(false)
			.default_width(640.0)
			.default_height(256.0)
			.show(ctx, |ui| self.ui(ui, state, system));
	}

	fn ui(&mut self, ui: &mut egui::Ui, state: &Arc<RwLock<Project>>, system: &mut SystemState) {
		let project = state.read().unwrap();
		egui::TopBottomPanel::top("mixer_menu").show_inside(ui, |ui| {
			ui.horizontal(|ui| {
				if ui.button(strings::MIXER_NEW_CHANNEL).clicked() {
					system.dispatch(UiEvent::AddChannel);
				}

				ui.label(format!("{}", project.mixer.channels.len()));
			});
		});

		egui::SidePanel::right("mixer_effects")
			.min_width(160.0)
			.resizable(false)
			.show_inside(ui, |ui| {
				if let None = project.mixer.selected_channel {
					return;
				}
				let selected_channel_index = project.mixer.selected_channel.unwrap();
				let channel = &project.mixer.channels[selected_channel_index];

				ui.scope(|ui| {
					let label = egui::Label::new(&channel.name);
					ui.add(label);
				});

				// For now, only list effect names
				for effect in &channel.effects {
					ui.label(format!(
						"{} (by {}",
						effect.get_info().name,
						effect.get_info().vendor
					));
				}
			});

		egui::ScrollArea::horizontal().show(ui, |ui| {
			ui.horizontal(|ui| {
				project
					.mixer
					.channels
					.iter()
					.enumerate()
					.for_each(|(idx, c)| view(ui, c, idx, system));
			});
		});
	}

	fn toggle_shortcut(&self) -> Option<egui::KeyboardShortcut> {
		Some(egui::KeyboardShortcut::new(
			egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
			egui::Key::M,
		))
	}
}

fn view_contents(ui: &mut egui::Ui, channel: &Channel, index: usize, system: &mut SystemState) {
	let mut channel_name = channel.name.clone();
	let mut channel_panning = channel.panning;
	let mut channel_volume = channel.volume;
	let mut channel_muted = channel.muted;
	let mut channel_selected = false;

	if ui.button("Select").clicked() {
		channel_selected = true;
	}

	ui.add(
		egui::TextEdit::singleline(&mut channel_name)
			.desired_width(64.0)
			.font(egui::TextStyle::Small)
			.min_size(egui::Vec2::new(64.0, 12.0)),
	);

	// Panning
	if ui
		.add(
			egui_extras_xt::knobs::AudioKnob::new(&mut channel_panning)
				.range(-1.0..=1.0)
				.spread(0.75)
				.drag_length(4.0)
				.animated(false)
				.shape(egui_extras_xt::common::WidgetShape::Circle),
		)
		.double_clicked()
	{
		// Reset pan to 0%
		system.dispatch(UiEvent::ChannelPanning {
			channel_index: index,
			panning: 0.0,
		});
	}

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
			system.dispatch(UiEvent::RemoveChannel {
				channel_index: index,
			});
		}
	}

	if channel_name != channel.name {
		system.dispatch(UiEvent::ChannelName {
			channel_index: index,
			name: channel_name,
		});
	}

	if channel_panning != channel.panning {
		system.dispatch(UiEvent::ChannelPanning {
			channel_index: index,
			panning: channel_panning,
		});
	}

	if channel_volume != channel.volume {
		system.dispatch(UiEvent::ChannelVolume {
			channel_index: index,
			volume: channel_volume,
		});
	}

	if channel_muted != channel.muted {
		system.dispatch(UiEvent::ChannelMuted {
			channel_index: index,
			muted: channel_muted,
		});
	}

	if channel_selected {
		system.dispatch(UiEvent::SelectChannel {
			channel_index: Some(index),
		})
	}
}

fn view(ui: &mut egui::Ui, channel: &Channel, index: usize, system: &mut SystemState) {
	ui.group(|ui| {
		ui.allocate_ui_with_layout(
			egui::vec2(64.0, 256.0),
			egui::Layout::top_down_justified(egui::Align::Center),
			|ui| view_contents(ui, channel, index, system),
		);
	});
}
