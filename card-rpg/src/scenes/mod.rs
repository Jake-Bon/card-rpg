use crate::events::event_subsystem::GameEvent;

pub mod battle;
pub mod overworld;

pub trait Scene {
	fn handle_input(&mut self, event: GameEvent);

	fn render(&mut self) -> Result<(), String>;
}