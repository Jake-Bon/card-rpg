extern crate sdl_rust;

use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const CAM_W: u32 = 720;
const CAM_H: u32 = 720;

// Fills the given WindowCanvas wincan with the given Color fill_color
pub fn fill_screen(wincan: &mut WindowCanvas, fill_color: Color) -> Result<(), String> {

    wincan.set_draw_color(fill_color);
    wincan.clear();
    Ok(())

}

// Draws sprite stretched to fit the entire window
pub fn draw_sprite_to_fit(wincan: &mut WindowCanvas, sprite_path: &str) -> Result<(), String> {
    
    let texture_creator = wincan.texture_creator();
    let portrait = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    let sprite_info = portrait.query();
    let img = Rect::new(0, 0, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    wincan.copy(&portrait, img, Rect::new(0, 0, CAM_W, CAM_H))?;
    
    Ok(())
}

// Draws sprite at the given location
//pub fn draw_sprite(wincan: &mut WindowCanvas, sprite_path: &str, x_pos: i32, y_pos: i32) -> Result<(), String> {
    
    //let texture_creator = wincan.texture_creator();
    //let portrait = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    //let sprite_info = portrait.query();
    //let img = Rect::new(x_pos, y_pos, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    //wincan.copy(&portrait, img, Rect::new(0, 0, CAM_W, CAM_H))?;
    
    //Ok(())
//}
