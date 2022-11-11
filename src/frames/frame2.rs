use std::thread;
use std::time::Duration;

use eframe::egui;
use super::Drawable;
use super::GenAppData;

pub struct Frame2 {
	name: String
}

impl Frame2 {
	pub fn new(name: &str) -> Self {
		Self {
			name: String::from(name)
		}
	}
}

impl Drawable for Frame2 {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	fn redraw(&mut self, ctx: &egui::Context, gen_data: &GenAppData) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Frame 2!");
		});
	}

	fn open(&mut self, gen_data: &GenAppData) {
		println!("[FRAME2][OPEN] requst data for frame.. with token: {}", gen_data.token);
		thread::sleep(Duration::from_millis(500));
		println!("[FRAME2][OPEN] parse and insert requested data.. complete!");
	}
}