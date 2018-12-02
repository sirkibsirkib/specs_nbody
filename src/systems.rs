use components::{Mass, Pos, Vel};
use specs::prelude::*;
use std::iter;

pub struct GravityForce;
impl<'a> System<'a> for GravityForce {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Mass>,
        WriteStorage<'a, Vel>,
    );

    fn run(&mut self, (pos, mass, mut vel): Self::SystemData) {
        for (me_p, me_m, mut me_v) in (&pos, &mass, &mut vel).join() {
            for (p, m) in (&pos, &mass).join() {
                let offset_toward_p = p.0 - me_p.0;
                let mut dist = offset_toward_p.length();
                if dist == 0.0 {
                    continue;
                }
                dist = dist.min(0.5);
                let force = 0.01 * (m.0 / me_m.0) / dist * dist;
                me_v.0 += offset_toward_p.with_length(force);
            }
        }
    }
}

pub struct PositionUpdate;
impl<'a> System<'a> for PositionUpdate {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (v, mut p) in (&vel, &mut pos).join() {
            p.0 += v.0;
        }
    }
}

pub struct Render {
    x: isize,
    y: isize,
    w: usize,
    h: usize,
    image: Vec<u8>,
}
impl Render {
    pub fn new(x: isize, y: isize, w: usize, h: usize) -> Self {
        Render {
            x,
            y,
            w,
            h,
            image: iter::repeat('o' as u8).take(w * h).collect(),
        }
    }
    pub fn clear(&mut self) {
        for c in self.image.iter_mut() {
            *c = ' ' as u8
        }
    }
    pub fn put(&mut self, x: f32, y: f32, what: u8) {
        if let Some(index) = self.index_for(x, y) {
            self.image[index] = what;
        }
    }
    pub fn index_for(&self, x: f32, y: f32) -> Option<usize> {
        let (x, y) = (x as isize, y as isize);
        if x < self.x || y < self.y {
            return None;
        }
        let (rx, ry) = ((x - self.x) as usize, (y - self.y) as usize);
        if rx >= self.w || ry >= self.h {
            None
        } else {
            Some(rx + ry * self.w)
        }
    }
    pub fn draw(&self) {
        print!("/");
        for _ in 0..self.w {
            print!("=");
        }
        println!("\\");
        for row in self.image[..].chunks(self.w) {
            print!("|");
            for c in row.iter() {
                print!("{}", *c as char);
            }
            println!("|");
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = ReadStorage<'a, Pos>;

    fn run(&mut self, pos: Self::SystemData) {
        self.clear();
        for p in (&pos).join() {
            let (x, y) = (p.0.x, p.0.y);
            self.put(x, y, 'o' as u8);
        }
        self.draw();
    }
}

// pub struct Debugger;
// impl<'a> System<'a> for Debugger {
//     type SystemData = (ReadStorage<'a, Pos>, ReadStorage<'a, Vel>);

//     fn run(&mut self, (pos, vel): Self::SystemData) {
//     	for (p, v) in (&pos, &vel).join() {
//     		println!("pos {:?} vel {:?}", p, v);
//     	}
//     }
// }
