use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::sdl_core::SDLCore;
use crate::game_manager::GameManager;
use crate::cards::battle_system;
use crate::video::text::FontManager;

mod game_manager;
mod video;
mod events;
mod scenes;
mod cards;

fn main() -> Result<(), String>{
    let mut sdl_core = SDLCore::init()?;
    let texture_manager = Rc::new(RefCell::new(TextureManager::new(&sdl_core.texture_creator)));
    let wincan = Rc::new(RefCell::new(sdl_core.wincan));
    let font_context = Rc::new(RefCell::new(sdl_core.font_context));
    let font_manager = Rc::new(RefCell::new(FontManager::init(wincan.clone(), font_context.clone(), &sdl_core.texture_creator)?));
    let mut game_manager = GameManager::init(&sdl_core.sdl_context, wincan, texture_manager, font_manager)?;

    //battle_system::test_libraries();

    game_manager.start_state_machine();

    Ok(())
}
