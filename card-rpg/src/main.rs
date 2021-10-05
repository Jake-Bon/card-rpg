use crate::game_manager::GameManager;

mod game_manager;
mod video;
mod events;

fn main() -> Result<(), String>{
    
    let game_manager = GameManager::init()?;
    game_manager.start_state_machine();

    Ok(())
}
