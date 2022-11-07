pub mod frame1;
pub mod frame2;

pub struct Frame {
	pub is_open: bool,
	pub frame: Box<dyn Drawable>
}

impl Frame {
	pub fn new(frame: Box<dyn Drawable>) -> Self {
		Self {
			is_open: false,
			frame: frame
		}
	}
}

pub trait Drawable {
    /// `&'static` so we can also use it as a key to store open/close state.
	fn name(&self) -> &'static str;

	/// Show windows, etc
	fn redraw(&mut self, ctx: &egui::Context);
}