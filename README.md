# Naive N-Body simulation using specs

I want to learn [Amethyst](https://www.amethyst.rs/) from the ground up, and really understand it. This little project is my first attempt to wrap my head around the entity component system at it's heart: [`specs`](https://crates.io/crates/specs), since all this gamedev stuff is all quite new to me.

## N-Body Simulation
The [N-Body simulation](https://en.wikipedia.org/wiki/N-body_simulation) problem seeks to simulate the movement of 'bodies' in a space in terms of change over discretized timesteps.
In this case, we observe the [universal gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation) between celestial bodies.

## ECS
[The book on `specs`](https://slide-rs.github.io/specs/) summarizes what an ECS is for quite nicely.
At it's heart, instead of bundling data for an entity into one compositional structure, we instead take the structure-of-arrays idea to the exteme. An entity is _nothing but an _identity_. An entity _keys_ into datastructures to access its relevant data. As such, data is grouped by structure type, rather than by being bundled together to compose an entity.

As the name suggests, an ECS system (no, the word "system is not redundant) can be viewed in terms of those three pillars. This is the breakdown of this project accordingly:
1. Entities: There are 5 bodies in the system
2. Components: Mass, Position, Velocity. Here we have a simple case where all 5 bodies have all the same components. Part of the point is that this is _not_ necessarily the case for an ECS in general. Indeed, that's part of the point.
3. Systems: 
    1. GravitationForce: updates velocity on each body
    2. PositionUpdate: updates position on each body
    3. Render: visualize bodies.
