use crate::{Body, Simulator, universe::G};
use nannou::prelude::{pow, DVec2};

pub struct Naive {}

impl Simulator for Naive {
    fn gravitation(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        let mut forces = vec![DVec2::default(); bodies.len()];
        for i in 0..bodies.len() {
            for j in i+1..bodies.len() {
                let offset = bodies[i].position-bodies[j].position;
                let dist = offset.length();
                let dir = offset/dist;

                let force = bodies[i].mass*bodies[j].mass*G/dist.powf(2.);
                forces[i] += -dir * force;
                forces[j] += dir * force;
            }
            bodies[i].update(forces[i], dt);
        }
    }
    fn collisions(&mut self, bodies: &mut Vec<Body>) {
        for i in 0..bodies.len() {
            for j in i+1..bodies.len() {
                let offset = bodies[i].position-bodies[j].position;
                let dist = offset.length();
                let dir = offset/dist;
                let col_radius = bodies[i].radius+bodies[j].radius;

                if dist < col_radius {
                    let col_delta = (col_radius-dist)/(bodies[i].mass+bodies[j].mass);
                    let body1 = bodies[i].clone();
                    let body2 = bodies[j].clone();

                    bodies[i].position += dir*col_delta*body2.mass;
                    bodies[j].position -= dir*col_delta*body1.mass;

                    bodies[i].collide(&body2);
                    bodies[j].collide(&body1);
                }
            }
        }
    }
    fn visualize(&self, draw: &nannou::Draw, bodies: &Vec<Body>) {
        
    }
}
