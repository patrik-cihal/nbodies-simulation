mod universe;
mod quadtree;
mod naive;
mod naive_multi_th;

use std::cmp::Ordering;
use std::time::Instant;

use ellipsoid::prelude::winit::event::{MouseButton, ElementState, MouseScrollDelta};
use naive::Naive;
use naive_multi_th::NaiveMultiTh;
use universe::stable_solar_system;
use crate::quadtree::QuadTree;
use crate::universe::{Body, big_bang};

use ellipsoid::{prelude::*, WindowEvent};

trait Simulator {
    fn gravitation(&mut self, bodies: &mut Vec<Body>, dt: f64);
    fn collisions(&mut self, bodies: &mut Vec<Body>);
    fn visualize(&self, bodies: &Vec<Body>) -> Vec<Shape>;
    fn update(&mut self, bodies: &Vec<Body>);
}

struct MyApp {
    bodies: Vec<Body>,
    offset: Vec2,
    zoom: f32,
    simulator: Box<dyn Simulator>,
    graphics: Graphics,
    mouse_position: Vec2,
    test_rot: f32,
}

impl App for MyApp {
    fn new(graphics: Graphics) -> Self {
        let bodies = stable_solar_system(10000, 500.);
        let simulator = Box::new(QuadTree::new(0.6));
        Self {bodies, offset: Vec2::default(), simulator, zoom: 1., graphics, mouse_position: Default::default(), test_rot: 0.}
    }

    fn graphics(&self) -> &Graphics {
        &self.graphics
    }

    fn graphics_mut(&mut self) -> &mut Graphics {
        &mut self.graphics
    }

    fn update(&mut self, dt: f32) {
        self.simulator.update(&self.bodies);
        self.simulator.gravitation(&mut self.bodies, dt as f64);
        self.simulator.collisions(&mut self.bodies);

        color_by_acceleration(&mut self.bodies);
        self.test_rot += dt;
    }

    fn input(&mut self, event: &ellipsoid::WindowEvent) -> bool {
        // check mouse wheel for zoom out and mouse position for translation

        match event {
            ellipsoid::WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.zoom *= 1. + y/10.;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.zoom *= 1. + pos.y as f32/100.;
                    }
                }
                true
            }
            ellipsoid::WindowEvent::MouseInput { state, button, .. } => {
                if *button == MouseButton::Left {
                    if *state == ElementState::Released {
                        self.offset -= (self.mouse_position) / self.zoom;
                    }
                    true
                } else {
                    false
                }
            }
            WindowEvent::CursorMoved { device_id, position, modifiers } => {
                let x = position.x as f32/self.graphics.window().inner_size().width as f32;
                let y = position.y as f32/self.graphics.window().inner_size().height as f32;
                self.mouse_position = vec2(x, -y) * 2. - vec2(1., -1.);
                false
            }
            _ => false
        }
    }

    fn draw(&mut self) {
        let instant = Instant::now();
        let background_color = Color::from_rgb(0.03, 0.03, 0.03);

        self.graphics.add_geometry(Shape::from_square().apply(GTransform::from_inflation(2.).translate(-vec2(0.5, 0.5))).color(background_color).into());
        let camera_gtransform = GTransform::from_inflation(self.zoom).translate(self.offset);
        for shape in self.simulator.visualize(&self.bodies) {
            self.graphics.add_geometry(shape.apply(camera_gtransform).into());
        }
        for body in &self.bodies {
            let body_shape = Shape::from_circle(7).apply(camera_gtransform.translate(body.position.as_vec2()).inflate(body.radius as f32)).color(body.color);
            self.graphics.add_geometry(body_shape.into());
        }

        println!("Drawing took {}ms", instant.elapsed().as_millis());
    }
}


fn main() {
    async_std::task::block_on(ellipsoid::run::<MyApp>());
}

fn color_by_acceleration(bodies: &mut Vec<Body>) {
    let mut accelerations = vec![];

    for body in &mut *bodies {
        accelerations.push(body.acceleration.length());
    }

    accelerations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let max_acceleration = accelerations[accelerations.len()/4*3]*4./3.;

    for body in &mut *bodies {
        let progress = (body.acceleration.length()/max_acceleration) as f32;
        body.color = Color::from_rgb(progress.powi(3)*0.2+0.8, 0.8, 0.8);
    }
}
