use std::collections::HashSet;

// use std::collections::BTreeSet;
use eframe::egui;
use frames::*;

mod frames;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(App::new(cc))));
}

struct App {
    gen_data: GenAppData,
    wins: Vec<Frame>
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            gen_data: GenAppData {token: String::from("FF00FF00")},
            wins: vec![
                Frame::new(Box::new(frame1::Frame1::new("frame1"))),
                Frame::new(Box::new(frame2::Frame2::new("frame2")))
            ]
        }
    }

    pub fn draw_content(&mut self, ctx: &egui::Context) {
        for win in &mut self.wins {
            if win.is_open() {
                win.frame.redraw(ctx, &self.gen_data);
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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