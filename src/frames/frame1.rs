use eframe::egui;
use super::Drawable;

pub struct Frame1 {

}

impl Frame1 {
	pub fn new() -> Self {
		Self {}
	}
}

impl Drawable for Frame1 {
	fn name(&self) -> &'static str {
		"frame1"
	}

	fn redraw(&mut self, ctx: &egui::Context) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Frame 1!");
		});
	}
}