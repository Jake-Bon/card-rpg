//Credits
use std::rc::Rc;
use std::cell::RefCell;
use std::process;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::mixer::Music;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;

// In the form of frames per slide
const TIMEOUT: u64 = 183;

pub struct Credits<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	slides: Vec<Rc<Texture<'a>>>,
	step: u64,
	music: Music<'a>,
	is_playing: bool,
}

impl<'a> Credits<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Result<Self, String> {
		let mut slides: Vec<Rc<Texture<'a>>> = Vec::new();

		let mut images = vec![
							"assets/credits/david.png","assets/credits/derek.png",
							"assets/credits/emilio.png","assets/credits/gabe.png",
							"assets/credits/jacob.png","assets/credits/louisa.png",
							"assets/credits/max.png","assets/credits/merrick.png"];
		for image in images.iter() {
			slides.push(texture_manager.borrow_mut().load(image).unwrap());
		}

		let mut step = 0;

		let frequency = 44100;
    	let format = AUDIO_S16LSB;
    	let channels = DEFAULT_CHANNELS;
    	let chunk_size = 1024;
    	sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    	let _mixer_context = sdl2::mixer::init(InitFlag::OGG)?;

		let music = Music::from_file("assets/music/MAP.ogg")?;

		Ok(Credits{
			wincan,
			event_system,
			slides,
			step,
			music,
			is_playing: false,
		})
	}
}

impl Scene for Credits<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		// Nothing... for now
	}

	fn render(&mut self) -> Result<(), String> {
		let mut wincan = self.wincan.borrow_mut();
		wincan.clear();

		if !self.is_playing{
			self.music = Music::from_file("assets/music/ADVENTURE.ogg")?;
			//self.music.play(1);
			self.music.fade_in_from_pos(1,1000,152.5);
			self.is_playing = true;
		}

		let curr_index =
			if self.step > TIMEOUT*8 {
				// Should quit instead
				self.step = 0;
        sdl2::mixer::Music::fade_out(1000);
				self.is_playing = false;
				self.event_system.borrow().change_scene(0).unwrap();
				process::exit(0);
				println!("QUIT");
				0
			}
			else if self.step > TIMEOUT*7 {
				7
			}
			else if self.step > TIMEOUT*6 {
				6
			}
			else if self.step > TIMEOUT*5 {
				5
			}
			else if self.step > TIMEOUT*4 {
				4
			}
			else if self.step > TIMEOUT*3 {
				3
			}
			else if self.step > TIMEOUT*2 {
				2
			}
			else if self.step > TIMEOUT {
				1
			}
			else {
				0
			};

		self.step += 1;

		crate::video::gfx::fill_screen(&mut wincan, Color::RGBA(0,0,0,255));
		crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.slides[curr_index]);
		wincan.present();

		Ok(())

	}
}
