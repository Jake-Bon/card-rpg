use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
//use sdl2::render::TextureQuery;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::rect::Rect;
//use sdl2::render::WindowCanvas;

use crate::game_manager::TextureManager;

use sdl2::ttf::Sdl2TtfContext;

//pub const TEXT_FONT = "";

pub struct FontManager<'l>{
    //wincan: Rc<RefCell<WindowCanvas>>,
    font_context: Rc<RefCell<Sdl2TtfContext>>,
    texture_creator: &'l TextureCreator<WindowContext>,
    text_cache: HashMap<String, Rc<Texture<'l>>>,
    
}

impl <'l> FontManager <'l> {
    
    pub fn init(wincan: Rc<RefCell<WindowCanvas>>, font_context: Rc<RefCell<Sdl2TtfContext>>, texture_creator: &'l TextureCreator<WindowContext>) -> Result<Self, String>{
        Ok(FontManager {
                //wincan,
                font_context,
                texture_creator,
                text_cache: HashMap::new(),   
            }
        )
    }
    
    pub fn draw_text(&mut self, wincan: &mut WindowCanvas, text: &str, (x_pos, y_pos): (i32, i32)) -> Result<(), String> {
        
        //let fontctxt = self.font_context.borrow_mut();
        //let font = fontctxt.load_font("calibri.ttf", 32)?;
        
        //let temp_surface = font.render(text);
        
        //println!("inside draw_text, about to try and set the font");
        
        let text_texture = self.create_text_texture("calibri.ttf", text)?;
        let texture_info = text_texture.query();
        //println!("text texture width and height: {} , {}", texture_info.width, texture_info.height);
        
        wincan.copy(&text_texture, Rect::new(0, 0, texture_info.width, texture_info.height), Rect::new(x_pos, y_pos, texture_info.width, texture_info.height));
        
        Ok(())
        
    }
    
    fn create_text_texture(&mut self, font_path: &str, text: &str) -> Result<Rc<Texture<'l>>, String> {
        
        //println!("inside create_text_texture()");
        
        self.text_cache.get(text).cloned().map_or_else(
			|| {
				//let texture = Rc::new(self.loader.load_texture(path)?);
				let fontctxt = self.font_context.borrow_mut();
				
				//println!("borrowed the font context successfully, about to try and set a font");
				
                let font = fontctxt.load_font("assets/fonts/Roboto-Regular.ttf", 32)?;
                
                //println!("loaded the font correctly");
                
                let temp_surface = font.render(text).solid(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;
                
                let font_texture = Rc::new(temp_surface.as_texture(&self.texture_creator).unwrap());//self.texture_creator.create_texture_from_surface(&temp_surface);
				
				self.text_cache.insert(text.to_string(), font_texture.clone());
				Ok(font_texture)
			},
			Ok,
		)
    }
    
}
