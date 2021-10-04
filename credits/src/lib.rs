extern crate sdl2;

use sdl2::rect::Rect;

pub struct SDLCore{
    sdl_cxt: sdl2::Sdl,
    pub wincan: sdl2::render::WindowCanvas,
    pub event_pump: sdl2::EventPump,
    pub cam: Rect,
}

impl SDLCore{
    pub fn init(title: &str,vsync: bool, width: u32, height: u32) -> Result<SDLCore, String>{
        let sdl_cxt = sdl2::init()?;
        let video_subsys = sdl_cxt.video()?;
        let window = video_subsys.window(title,width,height).build().map_err(|e| e.to_string())?;
        let wincan = window.into_canvas().accelerated();

        let wincan = if vsync{
            wincan.present_vsync()
        }else{
            wincan
        };

        let wincan = wincan.build().map_err(|e| e.to_string())?;
        let event_pump = sdl_cxt.event_pump()?;

        let cam = Rect::new(0,0,width,height);

        Ok(SDLCore{sdl_cxt,wincan,event_pump,cam})
    }
}

pub trait Game {
    fn init() -> Result<Self,String> where Self:Sized;
    fn run(&mut self) -> Result<(),String>;
}

pub fn runner<F,G>(desc:&str,initter: F) where F: Fn() -> Result<G,String>,G: Game{
    println!("\nRunning {}",desc);
    println!("\nInit...");

    match initter(){
        Err(e) => println!("\nFailed to init: {}",e),
        Ok(mut d) => {
            println!("\nDone");
            println!("\nRunning...");
            match d.run(){
                Err(e) => println!("\nRun error: {}",e),
                Ok(_) => println!("\nDone successfully"),
            };
        },
    };
}
