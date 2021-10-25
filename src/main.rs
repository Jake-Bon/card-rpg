use std::rc::Rc;
use std::cell::RefCell;

use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::sdl_core::SDLCore;
use crate::game_manager::GameManager;
use crate::cards::card_system;

mod game_manager;
mod video;
mod events;
mod scenes;
mod cards;

fn main() -> Result<(), String>{
    let mut sdl_core = SDLCore::init()?;
    let texture_manager = Rc::new(RefCell::new(TextureManager::new(&sdl_core.texture_creator)));
    let wincan = Rc::new(RefCell::new(sdl_core.wincan));
    let mut game_manager = GameManager::init(&sdl_core.sdl_context, wincan, texture_manager)?;
    println!("DEMO OF CARD SYSTEM");
    println!("-------------------");
    card_system::demo_card_system();
    game_manager.start_state_machine();

    Ok(())
}
