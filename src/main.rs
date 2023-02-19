#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::application::System;
use resources::strings;

mod application;
mod data;
mod resources;
mod utilities;

pub fn main() -> Result<(), eframe::Error> {
	let options = eframe::NativeOptions {
		min_window_size: Some(egui::vec2(640.0, 480.0)),
		initial_window_size: Some(egui::vec2(1280.0, 720.0)),
		centered: true,
		..Default::default()
	};

	eframe::run_native(
		strings::WINDOW_TITLE,
		options,
		Box::new(|cc| Box::new(System::new(cc))),
	)
}
