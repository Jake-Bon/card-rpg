use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct VideoSystem {
	wincan: WindowCanvas,
}

impl VideoSystem {

	pub fn update(&mut self) {
		self.wincan.set_draw_color(Color::RGB(0, 255, 0));
		self.wincan.clear();
		self.wincan.present();
	}
	
	pub fn init(wincan: WindowCanvas) -> Self {

		VideoSystem {
			wincan,
		}
	}
}