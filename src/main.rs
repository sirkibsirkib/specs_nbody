extern crate specs;
use specs::prelude::*;
use specs::{Entity, World};


extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

use std::{thread, time, collections::{VecDeque, HashMap},};

mod pts;
use pts::Pt2;

mod components;
use components::*;

mod systems;
use systems::{GravityForce, PositionUpdate, Render};

mod actions;
use actions::*;

fn main() {
    // define world
    let mut world = World::new();
    let sleep_time = time::Duration::from_millis(50);

    // register components
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Mass>();

    //setup systems
    let mut graviy_force_system = GravityForce;
    let mut position_update_system = PositionUpdate;

    let (w, h) = (40, 20);
    let (x, y) = (-(w as isize) / 2, -(h as isize) / 2);
    let mut render_system = Render::new(x, y, w, h);

    // create some entities
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mass_distr = Uniform::new(1.0, 3.0);
    let x_distr = Uniform::new(x as f32, x as f32 + w as f32);
    let y_distr = Uniform::new(y as f32, y as f32 + h as f32);

    let x = world.create_entity()
        .with(Mass(mass_distr.sample(&mut rng)))
        .with(Pos(Pt2::new(
            x_distr.sample(&mut rng),
            y_distr.sample(&mut rng),
        ))).with(Vel(Pt2::NULL))
        .build();
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

    let mut action_q = VecDeque::new();
    let mut actors: HashMap<Entity, Box<Actor>> = HashMap::new();
    actors.insert(x, Box::new(BoosterAi::new()));

    loop {
        // PHASE 1: collect actions
        {
            let wrapped = ReadableState::wrap(&world);
            for (&e, mut actor) in actors.iter_mut() {
                actor.act(e, &mut action_q, wrapped);
            }
        }
        
        // PHASE 2: apply actions
        for action in action_q.drain(..) {
            match action {
                Action::BumpEntity(e, pt) => {
                    if let Some(ref mut vel) = world.write_storage::<Vel>().get_mut(e) {
                        vel.0 += pt;
                    } else {
                        panic!("BAD AI!");
                    }
                }
            }
        }

        // PHASE 3: systems
        graviy_force_system.run_now(&world.res);
        position_update_system.run_now(&world.res);
        world.maintain();

        render_system.run_now(&world.res);
        thread::sleep(sleep_time);
    }
}


struct TickId(usize);

