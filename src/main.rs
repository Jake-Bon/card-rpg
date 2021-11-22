use std::rc::Rc;
use std::cell::RefCell;

use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::sdl_core::SDLCore;
use crate::game_manager::GameManager;
use crate::video::text::FontManager;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

mod game_manager;
mod video;
mod events;
mod scenes;
mod cards;
mod ai;

fn main() -> Result<(), String>{
    let mut sdl_core = SDLCore::init()?; 
    let wincan = Rc::new(RefCell::new(sdl_core.wincan));    
    let texture_manager = Rc::new(RefCell::new(TextureManager::new(&sdl_core.texture_creator)));
    //let wincan = Rc::new(RefCell::new(sdl_core.wincan));
    let font_context = Rc::new(RefCell::new(sdl_core.font_context));
    let font_manager = Rc::new(RefCell::new(FontManager::init(wincan.clone(), font_context.clone(), &sdl_core.texture_creator)?));
    
    // draw a loading screen
    crate::video::gfx::fill_screen(&mut wincan.borrow_mut(), Color::RGB(0, 120, 150))?;
    font_manager.borrow_mut().draw_text_ext(&mut wincan.borrow_mut(), "assets/fonts/Roboto-Regular.ttf", 96, Color::RGB(0, 0, 0), "Loading...", (370, 310));
    wincan.borrow_mut().present();
    
    let mut game_manager = GameManager::init(&sdl_core.sdl_context, wincan, texture_manager, font_manager)?;

    //battle_system::test_libraries();

    game_manager.start_state_machine();

    Ok(())
}
