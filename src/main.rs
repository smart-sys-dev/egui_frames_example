use rand::distributions::{Alphanumeric, DistString};
use eframe::egui;

use frames::*;
mod frames;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|_| Box::new(App::new())));
}

struct App {
    gen_data: GenAppData,
    wins: Vec<Frame>
}

impl App {
    pub fn new() -> Self {
        Self {
            gen_data: GenAppData {token: String::from("FF00FF00")},
            wins: Vec::new()
        }
    }

    pub fn draw_content(&mut self, ctx: &egui::Context) {
        for win in &mut self.wins {
            if win.is_open() {
                win.frame.redraw(ctx, &self.gen_data);
            }
        }
    }

    // add new window in redraw loop, if frame with the same name was exists - it will be deleted
    pub fn new_window(&mut self, frame: Box<dyn Drawable>) {
        self.delete_window(frame.name());
        self.wins.push(Frame::new(frame));
    }

    // delete frame from redraw loop, frame data will be non recoverable
    pub fn delete_window(&mut self, name: &str) {
        for i in 0..self.wins.len() {
            if self.wins[i].frame.name() == name {
                self.wins.remove(i);
                return;
            }
        }
    }

    /// Open specefied by name window, and close others
    pub fn open_exact(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.open(&self.gen_data);
            } else {
                win.close();
            }
        }
    }

    pub fn open(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.open(&self.gen_data);
                break;
            }
        }
    }

    // make don't show window, but don't delete frame data, frame may be shown again with call 'open' func 
    pub fn close(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.close();
                break;
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("frame1: create").clicked() {
                let spec_data: i32 = rand::random();
                self.new_window(Box::new(frame1::Frame1::new("frame1", spec_data, &self.gen_data)));
                self.open("frame1");
            }
            if ui.button("frame2: create").clicked() {
                let spec_data = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
                self.new_window(Box::new(frame2::Frame2::new("frame2", &spec_data, &self.gen_data)));
                self.open("frame2");
            }
            if ui.button("frame1: delete").clicked() {
                self.delete_window("frame1");
            }
            if ui.button("frame2: delete").clicked() {
                self.delete_window("frame2");
            }

            ui.separator();

            if ui.button("frame1: open").clicked() {
                self.open("frame1");
            }
            if ui.button("frame2: open").clicked() {
                self.open("frame2");
            }
            if ui.button("frame1: close").clicked() {
                self.close("frame1");
            }
            if ui.button("frame2: close").clicked() {
                self.close("frame2");
            }
        });
        self.draw_content(ctx);
    }
}