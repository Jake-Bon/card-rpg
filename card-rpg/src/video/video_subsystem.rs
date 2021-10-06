use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct VideoSubsystem {
	wincan: WindowCanvas,
}

impl VideoSubsystem {
	pub fn init(wincan: WindowCanvas) -> Self {

		VideoSubsystem {
			wincan,
		}
	}

	pub fn update(&mut self) {
		self.wincan.set_draw_color(Color::RGB(0, 255, 0));
		self.wincan.clear();
		self.wincan.present();
	}
}