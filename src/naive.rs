use crate::{Body, Simulator, universe::G};
use nannou::prelude::pow;

pub struct Naive {}

impl Simulator for Naive {
    fn simulate(&mut self, bodies: &mut Vec<Body>) {
        let dt: f64 = 1.0 / 60.0;
        for i in 0..bodies.len() {
            for j in i+1..bodies.len() {
                let dir = bodies[i].position-bodies[j].position;
                let dist = dir.length();
                let force = bodies[i].mass*bodies[j].mass*G/(pow(dist, 2));
                bodies[i].apply_force(-dir/dist*force*dt);
                bodies[j].apply_force(dir/dist*force*dt);
            }
            bodies[i].update(dt);
        }
    }
}
