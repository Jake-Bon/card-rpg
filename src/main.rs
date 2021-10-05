//extern crate sdl_rust;

use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
//use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const TITLE: &str = "CardRPG Credits";
const CAM_W: u32 = 960;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 4000;

fn run(wincan: &mut WindowCanvas) -> Result<(),String>{
    let images = vec!["../assets/gabe.png","../assets/jacob.png","../assets/louisa.png","../assets/max.png"]; //Images

    for image in images.iter(){ //For every image

        let texture_creator = wincan.texture_creator();
        let portrait = texture_creator.load_texture(image)?;
        let info = portrait.query(); //Get size information about image
        let img = Rect::new(0,0,info.width,info.height); //Create rectangle the size of the image


        wincan.set_draw_color(Color::RGBA(0,0,0,255)); //Set color black
        wincan.copy(&portrait,img,Rect::new(0,0,CAM_W,CAM_H))?; //Place entire image (img) within medium sized rectangle (smallest_img)
        wincan.present();

        thread::sleep(Duration::from_millis(TIMEOUT));
        wincan.clear();
    }

    wincan.clear();
    Ok(())
}

fn main(){
    let sdl_cxt = sdl2::init(); //SDL Context, if failure, panic
    match sdl_cxt{
        Err(e) => panic!("Context Failed: {}",e),
        Ok(_) => ()
    };

    let video_subsys = sdl_cxt.unwrap().video(); //SDL Video, if failure, panic
    match video_subsys{
        Err(e) => panic!("Video Failed: {}",e),
        Ok(_) => ()
    };

    let window = video_subsys.unwrap().window(TITLE,CAM_W,CAM_H).build(); //SDL Window, if failure, panic
    match window{
        Err(e) => panic!("Window Failed: {}",e),
        Ok(_) => ()
    };
    let mut wincan = window.unwrap().into_canvas().accelerated(); //wincan must be mutable, no longer have lib self
    wincan = wincan.present_vsync();

    let mut wincan = wincan.build(); //SDL Context, if failure, panic
    match wincan{
        Err(e) => panic!("Canvas Failed: {}",e),
        Ok(_) => ()
    };
    run(&mut wincan.unwrap());
}








///////////////////////////////////

/*extern crate sdl2;

use sdl2::rect::Rect;

use std::time::Duration;
use std::thread;

//use sdl2::pixels::Color;
use sdl2::image::LoadTexture;

//use sdl_rust::SDLCore;
//use sdl_rust::Demo;

const TITLE: &str = "CardRPG Credits";
const CAM_W: u32 = 720;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 2500;

pub struct SDL01 {
	core: SDLCore,
}

impl Demo for SDL01 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL01{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let img1 = texture_creator.load_texture("images/img1.png")?;
		let img2 = texture_creator.load_texture("images/img2.png")?;
		let img3 = texture_creator.load_texture("images/img3.png")?;

		//self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		//self.core.wincan.clear();

		self.core.wincan.copy(&img1, None, None)?;
		self.core.wincan.present();
        thread::sleep(Duration::from_millis(TIMEOUT));
        
		self.core.wincan.clear();
		self.core.wincan.copy(&img2, None, None)?;
		self.core.wincan.present();
		thread::sleep(Duration::from_millis(TIMEOUT));
        
		self.core.wincan.copy(&img3, None, None)?;
		self.core.wincan.present();
		thread::sleep(Duration::from_millis(TIMEOUT));
		


		Ok(())
	}
}

fn main() {
	runner(TITLE, SDL01::init);
}

/* SDL */

pub struct SDLCore {
	//sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
	pub cam: Rect,
}

impl SDLCore {
	pub fn init(
		title: &str,
		vsync: bool,
		width: u32,
		height: u32,
	) -> Result<SDLCore, String>
	{
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;

		let window = video_subsys.window(title, width, height)
			.build()
			.map_err(|e| e.to_string())?;

		let wincan = window.into_canvas().accelerated();

		// Check if we should lock to vsync
		let wincan = if vsync {
			wincan.present_vsync()
		}
		else {
			wincan
		};
		
		let wincan = wincan.build()
			.map_err(|e| e.to_string())?;

		let event_pump = sdl_cxt.event_pump()?;

		let cam = Rect::new(0, 0, width, height);

		Ok(SDLCore{
			//sdl_cxt,
			wincan,
			event_pump,
			cam,
		})
	}
}

pub trait Demo {
	fn init() -> Result<Self, String> where Self: Sized;
	fn run(&mut self) -> Result<(), String>;
}

pub fn runner<F, D>(desc: &str, initter: F)
	where
		F: Fn() -> Result<D, String>,
		D: Demo,
{
	println!("\nRunning {}:", desc);
	print!("\tInitting...");
	match initter() {
		Err(e) => println!("\n\t\tFailed to init: {}", e),
		Ok(mut d) => {
			println!("DONE");

			print!("\tRunning...");
			match d.run() {
				Err(e) => println!("\n\t\tEncountered error while running: {}", e),
				Ok(_) => println!("DONE\nExiting cleanly"),
			};
		},
	};
}*/
