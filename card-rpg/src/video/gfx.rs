//extern crate sdl_rust;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game_manager::TextureManager;

const CAM_W: u32 = 720;
const CAM_H: u32 = 720;

// Fills the given WindowCanvas wincan with the given Color fill_color
pub fn fill_screen(wincan: &mut WindowCanvas, fill_color: Color) -> Result<(), String> {

    wincan.set_draw_color(fill_color);
    wincan.clear();
    Ok(())

}

// Draws sprite stretched to fit the entire window
pub fn draw_sprite_to_fit(wincan: &mut WindowCanvas, sprite_texture: &'_ Texture) -> Result<(), String> {
    
    //let texture_creator = wincan.texture_creator();
    //let sprite_texture = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    let sprite_info = sprite_texture.query();
    let img = Rect::new(0, 0, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    wincan.copy(&sprite_texture, img, Rect::new(0, 0, CAM_W, CAM_H))?;
    
    Ok(())
}

// Draws sprite stretched to fit a defined set of dimensions at the given coordinates.
// Specifically, draws the given sprite stretched to fit a new_width by new_height area.
pub fn draw_sprite_to_dims(wincan: &mut WindowCanvas, sprite_texture: &'_ Texture, new_width: u32, new_height: u32, x_pos: i32, y_pos: i32) -> Result<(), String> {
    
    //let texture_creator = wincan.texture_creator();
    //let sprite_texture = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    //let sprite_info = sprite_texture.query();
    //let img = Rect::new(0, 0, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    wincan.copy(&sprite_texture, None, Rect::new(x_pos, y_pos, new_width, new_height))?;
    
    Ok(())
}

// Draws sprite at the given location
// Uses the dimensions of the texture itself, doesn't do any resizing
pub fn draw_sprite(wincan: &mut WindowCanvas, sprite_texture: &'_ Texture, x_pos: i32, y_pos: i32) -> Result<(), String> {
    
    //let texture_creator = wincan.texture_creator();
    //let portrait = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    let sprite_info = sprite_texture.query();
    //let img = Rect::new(x_pos, y_pos, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    wincan.copy(&sprite_texture, None, Rect::new(x_pos, y_pos, sprite_info.width, sprite_info.height))?;
    
    Ok(())
}
