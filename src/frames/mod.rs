pub mod frame1;
pub mod frame2;

pub struct Frame {
	is_open: bool,
	
	pub frame: Box<dyn Drawable>
}

/// Contains common fields for all frames and App state (current point_id ?)
pub struct GenAppData {
	pub token: String
	
	// And others
}

impl Frame {
	pub fn new(frame: Box<dyn Drawable>) -> Self {
		Self {
			is_open: false,
			frame: frame
		}
	}

	pub fn open(&mut self, gen_data: &GenAppData) {
		self.frame.open(gen_data);
		self.is_open = true;
	}

	pub fn close(&mut self) {
		self.is_open = false;
	}

	pub fn is_open(&self) -> bool {
		self.is_open
	}

	// and any generic control functions
}

pub trait Drawable {
    /// `&'a` so we can also use it as a key to store open/close state.
	fn name<'a>(&'a self) -> &'a str;

	/// Show window, etc
	fn redraw(&mut self, ctx: &egui::Context, gen_data: &GenAppData);

	/// init window - load and handle data for view
	fn open(&mut self, gen_data: &GenAppData);
}