extern crate sdl_rust;

use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const TITLE: &str = "Credits Sequence";
const CAM_WANDH: u32 = 720; //720x720
const TIMEOUT: u64 = 3000;

fn smallest_img(img: TextureQuery)->Rect{ //Simple Scaling Function. Small -> Medium and Large -> Small conversion
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


fn run(wincan: &mut WindowCanvas) -> Result<(),String>{
    let images = vec!["images/cheese.jpg","images/Jacob_big.png","images/100.jpg"]; //Images
    //let names = vec!["Cheesey Cheesington","Jacob Bonhomme","Timmy Turner"]; //Corresponding Names

    for image in images.iter(){ //For every image

        let texture_creator = wincan.texture_creator();
        let portrait = texture_creator.load_texture(image)?;
        let info = portrait.query(); //Get size information about image
        let img = Rect::new(0,0,info.width,info.height); //Create rectangle the size of the image


        wincan.set_draw_color(Color::RGBA(0,0,0,255)); //Set color black
        wincan.copy(&portrait,img,Rect::new(0,0,720,720))?; //Place entire image (img) within medium sized rectangle (smallest_img)
        wincan.present();
        //println!("Current Image: {}",names[i]);

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

    let window = video_subsys.unwrap().window(TITLE,CAM_WANDH,CAM_WANDH).build(); //SDL Window, if failure, panic
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
