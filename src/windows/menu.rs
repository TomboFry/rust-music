use super::{application::SystemState, mixer::Mixer, sampler::Sampler};
use egui::{Context, Modifiers, Ui};

pub fn draw_application_menu(ctx: &Context, state: &mut SystemState) {
	egui::TopBottomPanel::top("application-menu-bar").show(ctx, |ui| {
		egui::menu::bar(ui, |ui| {
			file_menu_button(ui);
			add_menu_button(ui, state);

			ui.separator();

			ui.label("Tempo:");
			ui.add(
				egui::DragValue::new(&mut state.project.tempo)
					.clamp_range(40.0..=320.0)
					.suffix(" bpm"),
			);

			ui.separator();

			ui.label("Time Signature:");
			ui.add(
				egui::DragValue::new(&mut state.project.time_signature_numerator)
					.clamp_range(2..=16),
			);
			ui.label("/");
			ui.add(
				egui::DragValue::new(&mut state.project.time_signature_denominator)
					.clamp_range(2..=16),
			);
		});
	});
}

fn file_menu_button(ui: &mut Ui) {
	let quit_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::Q);
	// let reset_shortcut =
	// 	egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

	// NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
	// or else they would only be checked if the "File" menu was actually open!

	if ui.input_mut(|i| i.consume_shortcut(&quit_shortcut)) {
		std::process::exit(0);
	}

	ui.menu_button("File", |ui| {
		ui.set_min_width(200.0);
		ui.style_mut().wrap = Some(false);

		if ui
			.add(egui::Button::new("Exit").shortcut_text(ui.ctx().format_shortcut(&quit_shortcut)))
			.clicked()
		{
			ui.close_menu();
			std::process::exit(0);
		}
	});
}

fn add_menu_button(ui: &mut Ui, state: &mut SystemState) {
	ui.menu_button("Add", |ui| {
		ui.set_min_width(200.0);
		ui.style_mut().wrap = Some(false);

		if ui.add(egui::Button::new("Channel")).clicked() {
			let window = state.windows.windows.get_mut("Mixer").unwrap();
			let mixer: &mut Mixer = window.as_any().downcast_mut().unwrap();
			mixer.add_channel();

			ui.close_menu();
		}

		if ui.add(egui::Button::new("Sample(s)")).clicked() {
			let window = state.windows.windows.get_mut("Sampler").unwrap();
			let sampler: &mut Sampler = window.as_any().downcast_mut().unwrap();

			ui.close_menu();

			sampler.add_samples();
		}
	});
}
