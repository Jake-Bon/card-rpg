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
	set_online: u32,
	online_turn_event_id: u32,
	set_card_to_play: u32,
	push_card_to_battle: u32,
	poll_updates: u32,
	win_or_loss: u32,
	send_enemy: u32,
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
						    203 => {let stat = if(data1 as u32==1){
								  TurnPhase::NotInitOnlineP1
							  }else{
								  TurnPhase::NotInitOnlineP2
							  };
								  game_events.push(Some(GameEvent::SetClientTurn(stat)));
						    },
						    204 => { game_events.push(Some(GameEvent::OnlinePlay(data1 as u32)));},
							205 => { game_events.push(Some(GameEvent::PushCard(data1 as u32)));},
							206 => { game_events.push(Some(GameEvent::PollFromBattle()));},
							207 => { game_events.push(Some(GameEvent::WinOrLoss(data1 as u32)));}
							208 => { game_events.push(Some(GameEvent::SendEnemy(data1 as u32)));}
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
		let set_online = unsafe { event_subsystem.register_event().unwrap() };
		let online_turn_event_id = unsafe { event_subsystem.register_event().unwrap() };
		let set_card_to_play = unsafe { event_subsystem.register_event().unwrap() };
		let push_card_to_battle = unsafe { event_subsystem.register_event().unwrap() };
		let poll_updates = unsafe { event_subsystem.register_event().unwrap() };
		let win_or_loss = unsafe { event_subsystem.register_event().unwrap() };
		let send_enemy = unsafe { event_subsystem.register_event().unwrap() };

		Ok(EventSystem {
			event_pump,
			event_subsystem,
			scene_change_event_id,
			set_battler_npc_deck_event_id,
			set_online,
			online_turn_event_id,
			set_card_to_play,
			push_card_to_battle,
			poll_updates,
			win_or_loss,
			send_enemy
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

		let data = ((turn_data.turn_id as u32) << 16 ) + turn_data.card_ids as u32;

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

	pub fn set_online(&self, player: u32) -> Result<(), String> {

	    let setOnlineTurn = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.set_online,
	        code: 203,
	        data1: player as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(setOnlineTurn)?;
	    Ok(())

	}

	pub fn set_card_to_play(&self, card: u32) -> Result<(), String> {

	    let setOnlineTurn = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.set_card_to_play,
	        code: 204,
	        data1: card as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(setOnlineTurn)?;
	    Ok(())

	}

	pub fn push_card_to_battle(&self, card: u32) -> Result<(), String> { //Push over the internet

	    let pushCard = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.push_card_to_battle,
	        code: 205,
	        data1: card as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(pushCard)?;
	    Ok(())

	}

	pub fn poll_for_updates(&self) -> Result<(), String> { //Poll over the internet

	    let pollUpdates = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.poll_updates,
	        code: 206,
	        data1: 0x5678 as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(pollUpdates)?;
	    Ok(())

	}

	pub fn set_win_or_loss(&self, stat: u32) -> Result<(), String> { //0=L,1=Neutral,2=W

	    let winOrLoss = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.win_or_loss,
	        code: 207,
	        data1: stat as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(winOrLoss)?;
	    Ok(())

	}

	pub fn send_enemy_to_battle(&self, id: u32) -> Result<(), String> { //0=L,1=Neutral,2=W

	    let sendEnemy = sdl2::event::Event::User {
	        timestamp: 0,
	        window_id: 0,
	        type_: self.send_enemy,
	        code: 208,
	        data1: id as *mut c_void,
	        data2: 0x5678 as *mut c_void, // could use this field to set the player's deck as well? To do both at once.
	    };

	    self.event_subsystem.push_event(sendEnemy)?;
	    Ok(())

	}
}

#[derive(Debug,PartialEq,Eq)]
pub enum GameEvent {
	WindowClose,
	SceneChange(u32),
	SetBattlerNPCDeck(u32),
	MouseClick(i32, i32),
	MouseHover(i32, i32),
	OnlineTurn(u16, u16),
	OnlinePlay(u32),
	SetClientTurn(TurnPhase),
	PushCard(u32),
	PollFromBattle(),
	WinOrLoss(u32),
	SendEnemy(u32),
	KeyPress(Keycode),
	KeyRelease(Keycode),
}
