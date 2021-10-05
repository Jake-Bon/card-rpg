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
    let images = vec!["images/credits-max.png","images/credits-louisa.png",
	"images/credits-jacob.png", "images/credits-david.png", 
	"images/credits-merrick.png", "images/credits-derek.png", 
	"images/credits-gabe.png", "images/credits-emilio.png"]; //Images

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
