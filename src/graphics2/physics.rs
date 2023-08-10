use std::ops::Add;
use sfml::system::{Time, Vector2f};
use crate::graphics2::core::{Thing, World};

pub(crate) struct Physics {
    accel_due_to_gravity: f32,
    grid_size: i32,
}

impl Physics {
    pub(crate) fn new(grid_size: i32) -> Physics {
        Physics {
            accel_due_to_gravity: -9.8 * 3.,
            grid_size: grid_size
        }
    }
    pub(crate) fn calculate(&self, world: &mut World, elapsedTime: Time) {
        let grid_tolerance = 5;
        world.things.iter_mut().for_each(|t | {
            let thing = t.as_mut();
            let xdiff = thing.get_position().x as i32 % self.grid_size;
            let ydiff = thing.get_position().y as i32 % self.grid_size;
            let ensure_the_bounce = thing.get_position().y < grid_tolerance as f32 * 2.;
            let not_on_a_grid_coord = xdiff > grid_tolerance && xdiff < (self.grid_size - grid_tolerance) &&  (ydiff > grid_tolerance && ydiff < (self.grid_size - grid_tolerance));
            let outside_the_grid = thing.get_position().x < 0. || thing.get_position().y < 0.;

            if (ensure_the_bounce || not_on_a_grid_coord || outside_the_grid) {
                let forces = self.calculate_forces_on(thing);
                let totalForce = forces.iter().fold(Vector2f::new(0., 0.), |a, b| { a.add(*b) });
                let accel = totalForce / thing.get_mass() as f32;
                thing.set_speed(thing.get_speed() + accel * elapsedTime.as_seconds());
                let new_pos = thing.get_position() + thing.get_speed();
                if (thing.get_position().x >= 0. && new_pos.y < 0. && thing.get_position().y >= 0.) {
                    let normal = Vector2f::new(0., -1.);
                    let dot_product = (thing.get_speed().x * 0.) + (thing.get_speed().y * 1.);
                    let new_speed_x = thing.get_speed().x + (2. * normal.x * dot_product) * thing.get_bounciness();
                    let new_speed_y = thing.get_speed().y + (2. * normal.y * dot_product) * thing.get_bounciness();
                    thing.set_speed(Vector2f::new(new_speed_x, new_speed_y));
                    thing.set_position(thing.get_position() + thing.get_speed());
                }
                else {
                    thing.set_position(thing.get_position() + thing.get_speed());
                }
            }
        });
    }

    fn calculate_forces_on(&self, thing: &dyn Thing) -> Vec<Vector2f>{
        let mut forces = Vec::new();
        forces.push(Vector2f::new(0., self.accel_due_to_gravity * thing.get_mass()));
        //forces.push(Vector2f::new(800., 0.));
        return forces;
    }
}
