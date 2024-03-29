use super::windows::{WindowName, Windows};
use crate::{
	data::SystemState,
	data::{Project, UiEvent},
	resources::strings,
	utilities::format::format_play_state,
};
use egui::{Context, Layout, Modifiers, Ui};
use egui_extras_xt::displays::{
	DisplayKind, DisplayMetrics, DisplayStylePreset, SegmentedDisplayWidget,
};
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;

pub fn draw_application_menu(
	ctx: &Context,
	frame: &mut eframe::Frame,
	windows: &mut Windows,
	system: &mut SystemState,
	state: &Arc<RwLock<Project>>,
) {
	egui::TopBottomPanel::top("application-menu-bar").show(ctx, |ui| {
		ui.with_layout(
			Layout::from_main_dir_and_cross_align(
				egui::Direction::LeftToRight,
				egui::Align::Center,
			)
			.with_cross_justify(true),
			|ui| {
				let project = &state.read().unwrap();
				let mut project_tempo = project.tempo;
				let mut project_time_signature_numerator = project.time_signature_numerator;
				let mut project_time_signature_denominator = project.time_signature_denominator;

				// Menu Buttons
				ui.scope(|ui| {
					menu_set_button_style(ui);

					file_button(ui, frame);
					add_button(ui, system);
					view_button(ui, windows);
				});

				// Project Toolbar
				ui.separator();

				ui.label(strings::PROJECT_TEMPO);
				ui.add(
					egui::DragValue::new(&mut project_tempo)
						.clamp_range(40.0..=320.0)
						.suffix(" bpm"),
				);

				ui.separator();

				ui.label(strings::PROJECT_TIME_SIGNATURE);
				ui.add(
					egui::DragValue::new(&mut project_time_signature_numerator).clamp_range(2..=16),
				);
				ui.label("/");
				ui.add(
					egui::DragValue::new(&mut project_time_signature_denominator)
						.clamp_range(2..=16),
				);

				ui.separator();

				ui.add(
					SegmentedDisplayWidget::new(DisplayKind::SevenSegment)
						.digit_height(24.0)
						.style_preset(DisplayStylePreset::DeLoreanRed)
						.push_string(format_play_state(&project.play_state))
						.show_apostrophes(false)
						.metrics(DisplayMetrics {
							colon_separation: 0.4,
							digit_spacing: 0.75,
							margin_horizontal: 1.0,
							margin_vertical: 0.2,
							..Default::default()
						}),
				);

				// TODO: Is there a nicer way to do this?
				if project_tempo != project.tempo {
					system.dispatch(UiEvent::ProjectTempo(project_tempo));
				}

				if project_time_signature_numerator != project.time_signature_numerator {
					system.dispatch(UiEvent::ProjectTimeSignatureNumerator(
						project_time_signature_numerator,
					));
				}

				if project_time_signature_denominator != project.time_signature_denominator {
					system.dispatch(UiEvent::ProjectTimeSignatureDenominator(
						project_time_signature_denominator,
					));
				}
			},
		);
	});
}

fn menu_set_button_style(ui: &mut Ui) {
	let style = ui.style_mut();
	style.spacing.button_padding = egui::vec2(8.0, 0.0);
	style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
	style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
	style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
	style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
}

fn file_button(ui: &mut Ui, frame: &mut eframe::Frame) {
	let quit_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::Q);

	if ui.input_mut(|i| i.consume_shortcut(&quit_shortcut)) {
		frame.close();
	}

	ui.menu_button(strings::MENU_LABEL_FILE, |ui| {
		ui.set_min_width(200.0);
		ui.style_mut().wrap = Some(false);

		if ui
			.add(egui::Button::new("Exit").shortcut_text(ui.ctx().format_shortcut(&quit_shortcut)))
			.clicked()
		{
			ui.close_menu();
			frame.close();
		}
	});
}

fn add_button(ui: &mut Ui, system: &mut SystemState) {
	ui.menu_button(strings::MENU_LABEL_ADD, |ui| {
		ui.set_min_width(200.0);
		ui.style_mut().wrap = Some(false);

		if ui.add(egui::Button::new("Channel")).clicked() {
			system.dispatch(UiEvent::ChannelAdd);
			ui.close_menu();
		}

		if ui.add(egui::Button::new("Sample(s)")).clicked() {
			// system.add_ui_event(UiEvent::AddSample { path: () });
			ui.close_menu();
		}
	});
}

fn view_button(ui: &mut Ui, windows: &mut Windows) {
	ui.menu_button(strings::MENU_LABEL_VIEW, |ui| {
		ui.set_min_width(200.0);
		ui.style_mut().wrap = Some(false);

		for name in WindowName::iter() {
			let window_open = windows.open.contains(&name);
			let window_icon = if window_open { "✔" } else { "  " };

			let mut button = egui::Button::new(format!("{window_icon} {name}"));

			if let Some(shortcut) = windows.windows[&name].toggle_shortcut() {
				button = button.shortcut_text(ui.ctx().format_shortcut(&shortcut));
			}

			if ui.add(button).clicked() {
				view_button_toggle_window(windows, name);
				ui.close_menu();
			}
		}
	});
}

pub fn view_button_toggle_window(windows: &mut Windows, window_name: WindowName) {
	let window_open = windows.open.contains(&window_name);
	if window_open {
		windows.open.remove(&window_name);
	} else {
		windows.open.insert(window_name);
	}
}
