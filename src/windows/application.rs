use super::{main_menu::draw_application_menu, Windows};
use crate::{
	data::{ui_event_handler, AudioEngine, AudioEngineEvent, SystemState},
	resources::{assets::setup_custom_fonts, UiEvent},
};
use rtrb::RingBuffer;
use std::{
	collections::VecDeque,
	sync::{Arc, RwLock},
};

pub struct System {
	pub state: Arc<RwLock<SystemState>>,
	pub engine: AudioEngine,
	pub ui_events: VecDeque<UiEvent>,
	pub windows: Windows,
}

impl System {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);

		let (upd_tx, upd_rx) = RingBuffer::<AudioEngineEvent>::new(64);
		let state_system = SystemState::new(upd_tx);
		let state_engine = Arc::new(RwLock::new(state_system));
		let state = Arc::clone(&state_engine);

		let engine = AudioEngine::new(&state_engine, upd_rx);

		Self {
			state,
			engine,
			ui_events: VecDeque::with_capacity(10),
			windows: Windows::default(),
		}
	}
}

impl eframe::App for System {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		draw_application_menu(ctx, &mut self.windows, &self.state, &mut self.ui_events);

		// TODO: Replace with rectangle to display custom colour or image
		egui::panel::CentralPanel::default().show(ctx, |_| {});

		self.windows.checkboxes(ctx);
		self.windows.windows(ctx, &self.state, &mut self.ui_events);

		ui_event_handler(&self.state, &mut self.ui_events);
	}
}
