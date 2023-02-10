pub mod application;
pub mod menu;
pub mod mixer;

pub trait Window {
	/// `&'static` so we can also use it as a key to store open/close state.
	fn name(&self) -> &'static str;

	/// Show windows, etc
	fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait View {
	fn ui(&mut self, ui: &mut egui::Ui);
}
