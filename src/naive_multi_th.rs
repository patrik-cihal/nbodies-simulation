use std::time::Instant;

use crate::{Body, Simulator, universe::G};
use ellipsoid::{Shape, prelude::glam::DVec2};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator, IntoParallelRefIterator};

pub struct NaiveMultiTh {}

impl Simulator for NaiveMultiTh {
    fn gravitation(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        let mut instant = Instant::now();

        let bodies_cloned = bodies.clone();
        bodies.par_iter_mut().enumerate().for_each(move |(i, body)| {
            let mut force = DVec2::default();
            for (j, other) in bodies_cloned.iter().enumerate() {
                if i == j {
                    continue;
                }
                let offset = body.position-other.position;
                let dist = offset.length();
                let dir = offset/dist;

                force += -dir * body.mass*other.mass*G/(dist*dist);
            }
            body.update(force, dt);
        });

        println!("Gravity: {}ms", instant.elapsed().as_millis());
    }
    fn collisions(&mut self, bodies: &mut Vec<Body>) {
        let bodies_cloned = bodies.clone();
        let mut instant = Instant::now();

        bodies.par_iter_mut().enumerate().for_each(move |(i, body)| {
            for (j, other) in bodies_cloned.iter().enumerate() {
                if i == j {
                    continue;
                }
                let offset = body.position-other.position;
                let dist = offset.length();
                let dir = offset/dist;
                let col_radius = body.radius+other.radius;

                if dist < col_radius {
                    let col_delta = (col_radius-dist)/(body.mass+other.mass);

                    body.position += dir*col_delta*other.mass;

                    body.collide(&other);
                }
            }
        });

        println!("Collisions: {}ms", instant.elapsed().as_millis());

    }
    fn visualize(&self, _bodies: &Vec<Body>) -> Vec<Shape> {
        vec![]
    }
    fn update(&mut self, _bodies: &Vec<Body>) {}
}
