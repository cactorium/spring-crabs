use cgmath::prelude::*;
use cgmath::Vector2;

use super::world::*;
use super::types::*;

pub fn tick(world: &mut World, env: &Environment, timestep: Unit) {
    for (_, ref mut mass) in world.masses.iter_mut() {
        mass.acc = Vector2::<Unit>::zero();
    }
    for (_, ref spring) in world.springs.iter() {
        let a = spring.endpoints[0];
        let b = spring.endpoints[1];
        let dx = world.masses[a].pos - world.masses[b].pos;
        let strain = dx.magnitude() - spring.length;
        let k = spring.stiffness.map_or(env.springiness, |x| x);
        let force = k * strain * dx.normalize();
        let m1 = world.masses[a].mass.map_or(1.0, |x| x);
        let m2 = world.masses[b].mass.map_or(1.0, |x| x);

        world.masses[a].acc = world.masses[a].acc + force/m1;
        world.masses[b].acc = world.masses[b].acc + force/m2;
    }

    for (_, ref mut mass) in world.masses.iter() {
        if mass.fixed {
            continue;
        }
        // TODO: simulate time step
    }

    for ref mut mass in world.masses.iter_mut() {
        // TODO: detect wall collisions
    }
}
