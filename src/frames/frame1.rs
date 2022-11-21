use std::thread;
use std::time::Duration;

use eframe::egui;
use super::Drawable;
use super::GenAppData;

pub struct Frame1 {
	name: String,
	frame1_data: i32
}

impl Frame1 {
	pub fn new(name: &str, spec_data: i32, gen_data: &GenAppData) -> Self {
		println!("[FRAME1][OPEN] requst data for frame.. with token: {}", gen_data.token);
		thread::sleep(Duration::from_millis(500));
		println!("[FRAME1][OPEN] parse and insert requested data.. complete!");
		Self {
			name: String::from(name),
			frame1_data: spec_data
		}
	}
}

impl Drawable for Frame1 {
	fn name<'a>(&'a self) -> &'a str {
		&self.name
	}

	fn redraw(&mut self, ctx: &egui::Context, gen_data: &GenAppData) {
		egui::Window::new(self.name())
		.open(&mut true)
		.show(ctx, |ui| {
			ui.heading("Hello World From Frame 1!");
			ui.heading(format!("My specific data(i32): {}", self.frame1_data));
		});
	}

	fn open(&mut self, gen_data: &GenAppData) {
		println!("Frame1 was open");
	}
}