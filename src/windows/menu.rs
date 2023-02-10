use egui::{Context, Modifiers, Ui};

pub fn draw_application_menu(ctx: &Context) {
	egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
		egui::menu::bar(ui, |ui| {
			file_menu_button(ui);
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
		ui.set_min_width(220.0);
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
