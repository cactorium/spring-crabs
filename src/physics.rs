use cgmath::prelude::*;
use cgmath::Vector2;

use super::world::*;
use super::types::*;

/*
pub fn tick(arena: &mut Arena, env: &Environment, timestep: Unit) {
    arena.with_mut(|masses: &mut [Mass], springs: &mut [Spring<MassIndex>]| {
        for ref mut mass in masses.into_iter() {
            mass.acceleration = Vector2::<Unit>::zero();
        }
        for ref spring in springs.into_iter() {
            // TODO: make more ergonomic by exposing ViewMut and implementing an Iterator
            // for ViewMut, so that this doesn't need to work in raw indices
            let a: usize = spring.endpoints[0].into();
            let b: usize = spring.endpoints[1].into();
            let dx = masses[a].position - masses[b].position;
            let strain = dx.magnitude() - spring.length;
            let k = spring.stiffness.map_or(env.springiness, |x| x);
            let force = k * strain * dx.normalize();
            let m1 = masses[a].mass.map_or(1.0, |x| x);
            let m2 = masses[b].mass.map_or(1.0, |x| x);

            masses[a].acceleration = masses[a].acceleration + force/m1;
            masses[b].acceleration = masses[b].acceleration + force/m2;
        }

        for ref mut mass in masses.into_iter() {
            if mass.fixed {
                continue;
            }
            // TODO: simulate time step
        }

        for ref mut mass in masses.into_iter() {
            // TODO: detect wall collisions
        }
    });
}
*/
