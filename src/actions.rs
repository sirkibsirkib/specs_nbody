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

struct BoosterAi {
	to_go: usize,
}
impl Actor for BoosterAi {
	fn act(&mut self, entity: Entity, q: &mut ActionEnqueuer, r: ReadableState) {
		self.to_go += 1;
		if self.to_go > 30 {
			let pos = r.get::<Pos>(entity).expect("AI expected POS");
			q.enqueue(Action::BumpEntity(entity, pos.0 * -2.0));
			self.to_go = 0;
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
	#[inline]
	pub fn get<T: specs::Component>(self, entity: Entity) -> Option<&'a T> {
		self.w.read_storage().get(entity)
	}
	pub fn wrap(w: &'a World) -> Self {
		Self { w }
	}
}