use sdl2::Sdl;
use sdl2::render::WindowCanvas;

const TITLE: &str = "Card RPG";
const WINDOW_HEIGHT: u32 = 720;
const WINDOW_WIDTH: u32 = 1080;

pub struct SDLCore {
	pub sdl_context: Sdl,
	pub wincan: WindowCanvas,
}

impl SDLCore {

	pub fn init() -> Result<Self, String> {
		let sdl_context = sdl2::init()?;
		let video_subsystem = sdl_context.video()?;

		let window = video_subsystem.window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
			.build()
			.map_err(|e| e.to_string())?;

		let wincan = window.into_canvas()
			.accelerated()
			.present_vsync()
			.build()
			.map_err(|e| e.to_string())?;

		Ok(SDLCore {
			sdl_context,
			wincan,
		})
	}
}