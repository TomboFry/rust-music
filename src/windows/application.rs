use super::{main_menu::draw_application_menu, Windows};
use crate::{
	data::{AudioEngine, AudioEngineEvent, SystemState},
	resources::assets::setup_custom_fonts,
};
use rtrb::RingBuffer;
use std::sync::{Arc, Mutex};

pub struct System {
	pub state: Arc<Mutex<SystemState>>,
	pub engine: AudioEngine,
	pub windows: Windows,
}

impl System {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);

		let (upd_tx, upd_rx) = RingBuffer::<AudioEngineEvent>::new(64);

		let state_system = SystemState::new(upd_tx);

		let state_engine = Arc::new(Mutex::new(state_system));
		let state = Arc::clone(&state_engine);

		let engine = AudioEngine::new(state_engine, upd_rx);

		Self {
			state,
			engine,
			windows: Windows::default(),
		}
	}
}

impl eframe::App for System {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		draw_application_menu(ctx, &mut self.windows, &mut self.state);

		egui::panel::CentralPanel::default().show(ctx, |_| {});

		self.windows.checkboxes(ctx);
		self.windows.windows(ctx, &mut self.state);
	}
}
