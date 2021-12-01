use crate::events::event_subsystem::GameEvent;

pub mod battle;
pub mod overworld;
pub mod menu;
pub mod online;
pub mod options;
pub mod credits;

pub trait Scene {
	fn handle_input(&mut self, event: GameEvent);

	fn render(&mut self) -> Result<(), String>;
}
