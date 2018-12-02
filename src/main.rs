extern crate specs;
use specs::prelude::*;
use specs::World;

extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

use std::{thread, time};

mod pts;
use pts::Pt2;

mod components;
use components::{Mass, Pos, Vel};

mod systems;
use systems::{GravityForce, PositionUpdate, Render};

fn main() {
    // define world
    let mut world = World::new();

    // register components
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Mass>();

    //setup systems
    let mut graviy_force_system = GravityForce;
    let mut position_update_system = PositionUpdate;
    let sleep_time = time::Duration::from_millis(50);

    let (w, h) = (40, 20);
    let (x, y) = (-(w as isize) / 2, -(h as isize) / 2);
    let mut render_system = Render::new(x, y, w, h);

    // create some entities
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mass_distr = Uniform::new(1.0, 3.0);
    let x_distr = Uniform::new(x as f32, x as f32 + w as f32);
    let y_distr = Uniform::new(y as f32, y as f32 + h as f32);
    for _ in 0..5 {
        world
            .create_entity()
            .with(Mass(mass_distr.sample(&mut rng)))
            .with(Pos(Pt2::new(
                x_distr.sample(&mut rng),
                y_distr.sample(&mut rng),
            ))).with(Vel(Pt2::NULL))
            .build();
    }

    loop {
        // send CTRL+C to kill

        graviy_force_system.run_now(&world.res);
        position_update_system.run_now(&world.res);
        world.maintain();
        render_system.run_now(&world.res);
        thread::sleep(sleep_time);
    }
}
