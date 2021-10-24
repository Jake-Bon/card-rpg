//extern crate sdl_rust;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game_manager::TextureManager;

pub const CAM_W: u32 = 720;
pub const CAM_H: u32 = 720;
pub const TILE_SIZE: u32 = 10;
pub const FRAME_DELAY: u32 = 30;

// Fills the given WindowCanvas wincan with the given Color fill_color
pub fn fill_screen(wincan: Rc<RefCell<WindowCanvas>>, fill_color: Color) -> Result<(), String> {

    let mut curcan = wincan.borrow_mut();

    curcan.set_draw_color(fill_color);
    curcan.clear();
    Ok(())

}

// Draws sprite stretched to fit the entire window
pub fn draw_sprite_to_fit(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture) -> Result<(), String> {
    
    //let texture_creator = wincan.texture_creator();
    //let sprite_texture = texture_creator.load_texture(sprite_path)?; // will panic if sprite_path is invalid
    let sprite_info = sprite_texture.query();
    let img = Rect::new(0, 0, sprite_info.width, sprite_info.height); // create a rectangle at the given position to copy the image onto.
    
    wincan.borrow_mut().copy(&sprite_texture, img, Rect::new(0, 0, CAM_W, CAM_H))?;
    
    Ok(())
}

// Draws sprite stretched to fit a defined set of dimensions at the given coordinates.
// Specifically, draws the given sprite stretched to fit a new_width by new_height area.
pub fn draw_sprite_to_dims(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture, (new_width, new_height): (u32, u32), (x_pos, y_pos): (i32, i32)) -> Result<(), String> {
    
    // draw the sprite at the given position, stretched to fit the given dimensions.
    wincan.borrow_mut().copy(&sprite_texture, None, Rect::new(x_pos, y_pos, new_width, new_height))?;
    
    Ok(())
}

// Draws sprite at the given location
// Uses the dimensions of the texture itself, doesn't do any resizing
pub fn draw_sprite(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture, (x_pos, y_pos): (i32, i32)) -> Result<(), String> {
    
    // queries the texture to get the sprite width and the sprite height
    let sprite_info = sprite_texture.query();

    // draw the sprite at the given position
    wincan.borrow_mut().copy(&sprite_texture, None, Rect::new(x_pos, y_pos, sprite_info.width, sprite_info.height))?;
    
    Ok(())
}

// Draws sprite from a sprite sheet at the given location
// The three tuples are as follows:
//      (sheet_x, sheet_y) = The position in the sprite sheet. Should be the top left corner of the sprite in the sprite sheet you want to draw.
//      (frame_width, frame_height) = The width and height of the sprite (or frame) you want to draw. Allows for rectangular sprites.
//      (x_pos, y_pos) =  The X and Y coordinates to draw the sprite at. Either can be negative.
pub fn draw_sprite_from_sheet(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture, (sheet_x, sheet_y): (i32, i32), (frame_width, frame_height): (u32, u32), (x_pos, y_pos): (i32, i32)) -> Result<(), String> {

    wincan.borrow_mut().copy(&sprite_texture, Rect::new(sheet_x, sheet_y, frame_width, frame_height), Rect::new(x_pos, y_pos, frame_width, frame_height))?;

    Ok(())
    
}

// Draws sprite repeatedly starting from a given position
// Uses the dimensions of the texture itself, doesn't do any resizing
//      cols is the number of times to repeat the texture horizontally (towards the right of the window)
//      rows is the number of times to repeat the texture vertically (towards the bottom of the window)
//      To draw a 2x2 cube, (cols, rows) should be (2, 2)
pub fn tile_sprite(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture, (start_x, start_y): (i32, i32), (cols, rows): (u32, u32)) -> Result<(), String> {
    
    let sprite_info = sprite_texture.query();
    
    for row in 0..rows{
        for col in 0..cols{
            wincan.borrow_mut().copy(&sprite_texture, None, Rect::new(
                                                        start_x + ((col * sprite_info.width) as i32),
                                                        start_y + ((row * sprite_info.height) as i32),
                                                        sprite_info.width,
                                                        sprite_info.height
                                                        ));
        };
    };
    
    Ok(())
    
}

// Draws a sprite from a sprite sheet repeatedly starting from a given position
// Uses the dimensions provided by frame_width and frame_height, doesn't do any resizing

// The four tuples are as follows:
//      (sheet_x, sheet_y) = The position in the sprite sheet. Should be the top left corner of the sprite in the sprite sheet you want to draw.
//      (frame_width, frame_height) = The width and height of the sprite (or frame) you want to draw. Allows for rectangular sprites.
//      (start_x, start_y) =  The X and Y coordinates to draw the sprite at. Either can be negative.
//      (cols, rows) =  cols is the number of times to repeat the texture horizontally (towards the right of the window)
//                      rows is the number of times to repeat the texture vertically (towards the bottom of the window)
pub fn tile_sprite_from_sheet(wincan: Rc<RefCell<WindowCanvas>>, sprite_texture: &'_ Texture, (sheet_x, sheet_y): (i32, i32), (frame_width, frame_height): (u32, u32), (start_x, start_y): (i32, i32), (cols, rows): (u32, u32)) -> Result<(), String> {

    for row in 0..rows{
        for col in 0..cols{
            wincan.borrow_mut().copy(
                &sprite_texture,
                Rect::new(
                    sheet_x,
                    sheet_y,
                    frame_width,
                    frame_height
                ),
                Rect::new(
                    start_x + ((col * frame_width) as i32),
                    start_y + ((row * frame_height) as i32),
                    frame_width,
                    frame_height
                )
            );
        };
    };
    
    Ok(())

}



