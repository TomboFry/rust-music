use egui::{Context, Modifiers, Ui};

pub fn draw_application_menu(ctx: &Context) {
	egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
		egui::menu::bar(ui, |ui| {
			file_menu_button(ui);
		});
	});
}

// Taken from https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/demo_app_windows.rs
// while I still work things out.
fn file_menu_button(ui: &mut Ui) {
	let organize_shortcut =
		egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
	let reset_shortcut =
		egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

	// NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
	// or else they would only be checked if the "File" menu was actually open!

	if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
		ui.ctx().memory_mut(|mem| mem.reset_areas());
	}

	if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
		ui.ctx().memory_mut(|mem| *mem = Default::default());
	}

	ui.menu_button("File", |ui| {
		ui.set_min_width(220.0);
		ui.style_mut().wrap = Some(false);

		if ui
			.add(
				egui::Button::new("Organize Windows")
					.shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
			)
			.clicked()
		{
			ui.ctx().memory_mut(|mem| mem.reset_areas());
			ui.close_menu();
		}

		if ui
			.add(
				egui::Button::new("Reset egui memory")
					.shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
			)
			.on_hover_text("Forget scroll, positions, sizes etc")
			.clicked()
		{
			ui.ctx().memory_mut(|mem| *mem = Default::default());
			ui.close_menu();
		}
	});
}
