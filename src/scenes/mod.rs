use crate::events::event_subsystem::GameEvent;

pub mod battle;
pub mod overworld;
pub mod menu;
<<<<<<< HEAD
pub mod online;
=======
pub mod credits;
>>>>>>> main

pub trait Scene {
	fn handle_input(&mut self, event: GameEvent);

	fn render(&mut self) -> Result<(), String>;
}
