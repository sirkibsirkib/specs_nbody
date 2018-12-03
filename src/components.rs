use pts::Pt2;
use specs::{Component, VecStorage, HashMapStorage};
use actions::{Actor, ActionEnqueuer, Action};
use specs::Entity;

#[derive(Debug)]
pub struct Mass(pub f32);
impl Component for Mass {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Pos(pub Pt2);
impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Vel(pub Pt2);
impl Component for Vel {
    type Storage = VecStorage<Self>;
}