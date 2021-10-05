mod gfx;

extern crate sdl_rust;

use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const TITLE: &str = "Credits Sequence";
const CAM_W: u32 = 720;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 3000;

fn main(){
    
    let sdl_cxt = sdl2::init();
    match sdl_cxt{
        Err(e) => panic!("Context Failed!: {}", e),
        Ok(_) => (),
    };
    
    let video_subsys = sdl_cxt.unwrap().video();
    match video_subsys{
        Err(e) => panic!("Window Failed!: {}", e),
        Ok(_) => (),
    };
    
    let window = video_subsys.unwrap().window(TITLE, CAM_W, CAM_H).build();
    match window{
        Err(e) => panic!("Canvas Failed!: {}", e),
        Ok(_) => (),
    }
    
    let mut wincan = window.unwrap().into_canvas().accelerated();
    wincan = wincan.present_vsync();
    
    let mut wincan = wincan.build(); // shadow using previous wincan
    match wincan{
        Err(e) => panic!("Canvas Failed!: {}", e), 
        Ok(_) => (),
    }
    
    let mut wincan = wincan.unwrap(); // shadow using previous wincan

    let images = vec!["../assets/gabe.png","../assets/jacob.png","../assets/louisa.png", "../assets/max.png"];
    
    for image in images.iter(){
    
        // clear the screen
        gfx::fill_screen(&mut wincan, Color::RGBA(0,0,0,255));
        
        gfx::draw_sprite_to_fit(&mut wincan, image);
        
        // draw frame to window
        wincan.present();
        
        println!("ok time to sleep zzz");
        
        // display frame for 3 seconds
        thread::sleep(Duration::from_millis(3000 as u64));
        
        println!("no more sleep");
    
    }
    
}
