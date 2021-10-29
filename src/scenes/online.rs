use std::rc::Rc;
use std::cell::RefCell;

use sdl2::render::WindowCanvas;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) { todo!() }

		fn render(&mut self) -> Result<(), String> { todo!() }

}