use cgmath::prelude::*;
use cgmath::Vector2;

use super::world::*;
use super::types::*;

pub fn mass_step(mass: &mut Mass, timestep: Unit) {
    // semi-implicit Euler time step
    let m = mass.mass.unwrap_or(1.0);
    mass.vel = mass.vel + mass.acc/m * timestep;
    mass.pos = mass.pos + mass.vel * timestep;
}

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
        let m1 = world.masses[a].mass.unwrap_or(1.0);
        let m2 = world.masses[b].mass.unwrap_or(1.0);

        world.masses[a].acc = world.masses[a].acc + force/m1;
        world.masses[b].acc = world.masses[b].acc + force/m2;
    }

    for (_, ref mut mass) in world.masses.iter_mut() {
        if mass.fixed {
            continue;
        }
        mass_step(mass, timestep);
    }

    for ref mut mass in world.masses.iter_mut() {
        // TODO: detect wall collisions
    }
}
