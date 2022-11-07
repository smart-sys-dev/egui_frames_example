use eframe::egui;
use super::Drawable;

pub struct Frame2 {

}

impl Frame2 {
	pub fn new() -> Self {
		Self {}
	}
}

impl Drawable for Frame2 {
	fn name(&self) -> &'static str {
		"frame2"
	}

	fn redraw(&mut self, ctx: &egui::Context) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Frame 2!");
		});
	}
}