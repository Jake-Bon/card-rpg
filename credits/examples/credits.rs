extern crate sdl_rust;

use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl_rust::SDLCore;
use sdl_rust::Game;
use sdl2::rect::Rect;
use sdl2::rect::Point;

const TITLE: &str = "Credits Sequence";
const CAM_WANDH: u32 = 720;
const TIMEOUT: u64 = 3000;

pub struct Credit{
    core: SDLCore
}

fn smallest_img(img: TextureQuery)->Rect{
    let mut x = img.width as f64;
    let mut y = img.height as f64;
    if x<y{
        while y>400.0{
            //println!("{}",x);
            x = x/1.1;
            y = y/1.1;
        }
        while y<250.0{
            //println!("{}",x);
            x = x*1.1;
            y = y*1.1;
        }
    }else{
        while x>400.0{
            //println!("{}",x);
            x = x/1.1;
            y = y/1.1;
        }
        while y<250.0{
            //println!("{}",x);
            x = x*1.1;
            y = y*1.1;
        }
    }
    return Rect::new(720/2-(x as i32)/2,720/2-(y as i32)/2,x as u32,y as u32);
}

impl Game for Credit{
    fn init() -> Result<Self, String>{
        let core = SDLCore::init(TITLE,true,CAM_WANDH,CAM_WANDH)?;
        Ok(Credit{core})
    }



    fn run(&mut self) -> Result<(),String>{
        let images = vec!["images/cheese.jpg","images/Jacob_big.png","images/100.jpg"];
        let names = vec!["Cheesey Cheesington","Jacob Bonhomme","Timmy Turner"];

        for i in 0..images.len(){

            let texture_creator = self.core.wincan.texture_creator();
            let portrait = texture_creator.load_texture(images[i])?;
            let info = portrait.query();
            let img = Rect::new(0,0,info.width,info.height);


            self.core.wincan.set_draw_color(Color::RGBA(0,0,0,255));
            self.core.wincan.copy(&portrait,img,smallest_img(info))?;
            self.core.wincan.present();
            println!("Current Image: {}",names[i]);

            thread::sleep(Duration::from_millis(TIMEOUT));
            self.core.wincan.clear();
        }

        self.core.wincan.clear();
        Ok(())
    }
}

fn main(){
    sdl_rust::runner(TITLE,Credit::init);
}
