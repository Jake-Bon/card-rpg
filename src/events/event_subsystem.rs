use std::os::raw::c_void;
use sdl2::{Sdl, EventPump, EventSubsystem};
use sdl2::keyboard::Keycode;
use sdl2::event::Event as SDL_Event;

use crate::cards::battle_enums::*;
use crate::scenes::online::TurnData;

pub struct EventSystem {
	event_pump: EventPump,
	event_subsystem: EventSubsystem,
	scene_change_event_id: u32,
	set_battler_npc_deck_event_id: u32,
	online_turn_event_id: u32,
}

impl EventSystem {

	pub fn update(&mut self) -> Vec<Option<GameEvent>> {

		let mut game_events:Vec<Option<GameEvent>> = Vec::new();
		for event in self.event_pump.poll_iter() {
			match event {
				SDL_Event::Quit{..} => game_events.push(Some(GameEvent::WindowClose)),
				SDL_Event::MouseButtonDown{x: x_pos, y: y_pos, ..} => game_events.push(Some(GameEvent::MouseClick(x_pos, y_pos))),
				//SDL_Event::MouseMotion{x: x_pos, y: y_pos, ..} => game_events.push(Some(GameEvent::MouseHover(x_pos, y_pos))), // I think we can uncomment this now?
				SDL_Event::KeyDown{keycode: Some(k), ..} => game_events.push(Some(GameEvent::KeyPress(k))),
				SDL_Event::KeyUp{keycode: Some(k), ..} => game_events.push(Some(GameEvent::KeyRelease(k))),
				// Need to create custom events like this. Differentiate events via each event's code value
				// Essentially, we're using one SDL event as many different events via the GameEvent enum. Each scene is acually being sent a value from the GameEvent enum, not the event directly from SDL
				SDL_Event::User{code: custom_event_code, data1: data1, ..} => {
				    match custom_event_code {
				        200 => { game_events.push(Some(GameEvent::SceneChange(data1 as u32))); },
				        201 => { game_events.push(Some(GameEvent::SetBattlerNPCDeck(data1 as u32))); },
				        202 => { game_events.push(Some(GameEvent::OnlineTurn((data1 as u32 >> 16) as u16, data1 as u16))); },
						    203 => { let stat = if(data1 as u32==1){
								  TurnPhase::NotInitOnlineP1
							  }else{
								  TurnPhase::NotInitOnlineP2
							  };
								  game_events.push(Some(GameEvent::SetClientTurn(stat)));

						    },
						    204 => { game_events.push(Some(GameEvent::OnlinePlay(data1 as u32)));},
				          _ => {},
				        }
				},
				//SDL_Event::User{code: scene_change_event_id, data1: scene_id, ..} => game_events.push(Some(GameEvent::SceneChange(scene_id as u32))),
				_ => game_events.push(None),
			}
		}

		return game_events;
	}

	pub fn init(sdl_context: &Sdl) -> Result<Self, String> {
		let event_pump = sdl_context.event_pump()?;
		let event_subsystem = sdl_context.event()?;

		let scene_change_event_id = unsafe { event_subsystem.register_event().unwrap() };
		let set_battler_npc_deck_event_id = unsafe { event_subsystem.register_event().unwrap() };
		let online_turn_event_id = unsafe { event_subsystem.register_event().unwrap() };

		Ok(EventSystem {
			event_pump,
			event_subsystem,
			scene_change_event_id,
			set_battler_npc_deck_event_id,
			online_turn_event_id,
		})
	}

	pub fn change_scene(&self, scene_id: u32) -> Result<(), String> {
		let event = sdl2::event::Event::User {
			timestamp: 0,
			window_id: 0,
			type_: self.scene_change_event_id,
			code: 200,
			data1: scene_id as *mut c_void,
			data2: 0x5678 as *mut c_void,
		};

		self.event_subsystem.push_event(event)?;
		Ok(())
	}

	pub fn receive_online(&self, mut turn_data: TurnData) -> Result<(), String> {

		let data = (turn_data.turn_id as u32) << 16 + turn_data.card_ids;

		let event = sdl2::event::Event::User {
			timestamp: 0,
			window_id: 0,
			type_: self.online_turn_event_id,
			code: 202,
			data1: data as *mut c_void,
			data2: 0x5678 as *mut c_void,
		};

		println!("Turn Data before: {:?}", turn_data);

		self.event_subsystem.push_event(event)?;
		Ok(())
	}

	pub fn set_battler_npc_deck(&self, deck_id: u32) -> Result<(), String> {

	    let setBattlerNpcDeckEvent = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.set_battler_npc_deck_event_id,
	        code: 201,
	        data1: deck_id as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    //println!("pushed the set_battler_npc_deck event to the event pump. \n{:#?}", setBattlerNpcDeckEvent);
	    //match self.event_subsystem.push_event(setBattlerNpcDeckEvent) {
	        //Ok(T) => println!("     the return value of the event subsystem was ()"),
	        //Err(E) => println!("{}", E),
	    //}
	    self.event_subsystem.push_event(setBattlerNpcDeckEvent)?;
	    Ok(())

	}
}

#[derive(Debug)]
pub enum GameEvent {
	WindowClose,
	SceneChange(u32),
	SetBattlerNPCDeck(u32),
	MouseClick(i32, i32),
	MouseHover(i32, i32),
	OnlineTurn(u16, u16),
	OnlinePlay(u32),
	SetClientTurn(TurnPhase),
	KeyPress(Keycode),
	KeyRelease(Keycode),
}
