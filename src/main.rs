#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use resources::strings::WINDOW_TITLE;

use crate::windows::application::SystemState;

mod resources;
mod windows;

pub fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		min_window_size: Some(egui::vec2(640.0, 480.0)),
		initial_window_size: Some(egui::vec2(1280.0, 720.0)),
		..Default::default()
	};

	eframe::run_native(
		WINDOW_TITLE,
		options,
		Box::new(|cc| Box::new(SystemState::new(cc))),
	)
}
