use pts::Pt2;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Mass(pub f32);
// impl Mass {
// 	pub fn new(inner: f32) -> Self {
// 		Mass(inner)
// 	}
// }
impl Component for Mass {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Pos(pub Pt2);
// impl Pos {
// 	pub fn new(inner: Pt2) -> Self {
// 		Pos(inner)
// 	}
// }
impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Vel(pub Pt2);
// impl Vel {
// 	pub fn new(inner: Pt2) -> Self {
// 		Vel(inner)
// 	}
// }
impl Component for Vel {
    type Storage = VecStorage<Self>;
}
