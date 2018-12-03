use std::collections::VecDeque;
use specs::{Entity, ReadStorage, World};
use pts::Pt2;
use components::*;

pub enum Action {
	BumpEntity(Entity, Pt2),
}

impl ActionEnqueuer for VecDeque<Action> {
	fn enqueue(&mut self, action: Action) {
		self.push_back(action);
	}
	
}

pub trait ActionEnqueuer {
	fn enqueue(&mut self, action: Action);
}

pub trait Actor {
	fn act(&mut self, entity: Entity, q: &mut ActionEnqueuer, r: ReadableState);
}

pub struct BoosterAi {
	to_go: usize,
}
impl BoosterAi {
	pub fn new() -> Self {
		Self {to_go: 0}
	}
}
impl Actor for BoosterAi {
	fn act(&mut self, entity: Entity, q: &mut ActionEnqueuer, r: ReadableState) {
		self.to_go += 1;
		if self.to_go > 20 {
			if let Some(vel) = r.read::<Vel>(entity).get(entity) {
				q.enqueue(Action::BumpEntity(entity, -2.0 * vel.0));
				self.to_go = 0;
			}
		}
	}
}


/*
World allows quite a lot of actions to try and fail using just `&World`
So this wrapper constrains the
*/
#[derive(Copy, Clone)]
pub struct ReadableState<'a> {
	w: &'a World
}
impl<'a> ReadableState<'a> {
	pub fn read<T: specs::Component>(self, entity: Entity) -> ReadStorage<'a, T> {
		self.w.read_storage()
	}
	pub fn wrap(w: &'a World) -> Self {
		Self { w }
	}
}
