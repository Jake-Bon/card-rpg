use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;

use sdl2::ttf::Sdl2TtfContext;

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
    
    // Basic text function, better for debugging / quick use
    //      Always draws white text with a font size of 32 in Roboto-Regular
    pub fn draw_text(&mut self, wincan: &mut WindowCanvas, text: &str, (x_pos, y_pos): (i32, i32)) -> Result<(), String> {
        
        let text_texture = self.get_text_texture("assets/fonts/Roboto-Regular.ttf", text, 32, Color::RGB(255, 255, 255))?;
        let texture_info = text_texture.query();
        
        wincan.copy(&text_texture, Rect::new(0, 0, texture_info.width, texture_info.height), Rect::new(x_pos, y_pos, texture_info.width, texture_info.height));
        
        Ok(())
        
    }
    
    // Text rendering function that allows for more customization of text
        // font_path - &str representation of the file path to the TrueType Font file (.ttf) file
        // font_size - The size of the text being drawn
        // text_color - The color of the text being drawn. Accepts an SDL Color, doesn't support alpha channel
        // text -  The text to draw. Will be one line of text, no line breaks
        // (x_pos, y_pos) - The x and y coordinates to start drawing the text at
    pub fn draw_text_ext(&mut self, wincan: &mut WindowCanvas, font_path: &str, font_size: u16, text_color: Color, text: &str, (x_pos, y_pos): (i32, i32)) -> Result<(), String> {
        
        let text_texture = self.get_text_texture(font_path, text, font_size, text_color)?;
        let texture_info = text_texture.query();

        wincan.copy(&text_texture, Rect::new(0, 0, texture_info.width, texture_info.height), Rect::new(x_pos, y_pos, texture_info.width, texture_info.height));
        
        Ok(())   
        
    }
    
    // Function that either returns the requested text texture from the cache, or creates and stores a new one
    // Works exactly like the TextureManager, but only stores textures that are words
    // Prevents creating new textures every frame
    fn get_text_texture(&mut self, font_path: &str, text: &str, font_size: u16, text_color: Color) -> Result<Rc<Texture<'l>>, String> {
        
        self.text_cache.get(text).cloned().map_or_else(
			|| {

				let fontctxt = self.font_context.borrow_mut();

                let font = fontctxt.load_font(font_path, font_size)?;

                let temp_surface = font.render(text).solid(text_color).map_err(|e| e.to_string())?;
                
                let font_texture = Rc::new(temp_surface.as_texture(&self.texture_creator).unwrap());
				
				self.text_cache.insert(format!("{}{}{}{}{}{}{}", font_path, text, font_size.to_string(), text_color.r.to_string(), text_color.g.to_string(), text_color.b.to_string(), text_color.a.to_string()), font_texture.clone());

				Ok(font_texture)
			},
			Ok,
		)
    }
    
}
