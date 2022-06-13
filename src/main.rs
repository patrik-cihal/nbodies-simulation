mod universe;
mod quadtree;
mod naive;

use std::cmp::Ordering;

use naive::Naive;
use nannou::prelude::*;
use universe::stable_solar_system;
use crate::quadtree::QuadTree;
use crate::universe::{Body, big_bang};

trait Simulator {
    fn gravitation(&mut self, bodies: &mut Vec<Body>, dt: f64);
    fn collisions(&mut self, bodies: &mut Vec<Body>);
    fn visualize(&self, draw: &Draw, bodies: &Vec<Body>);
    fn update(&mut self, bodies: &Vec<Body>);
}

struct Model {
    bodies: Vec<Body>,
    offset: Vec2,
    zoom: f32,
    simulator: Box<dyn Simulator>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}


fn model(app: &App) -> Model {
    app.new_window().mouse_wheel(mouse_wheel).build().unwrap();

    let win = app.main_window().rect();
    let bodies = big_bang(5000, (win.w()/2.) as f64,1.5);
    Model {bodies, offset: Vec2::default(), simulator: Box::new(QuadTree::new(1.5)), zoom: 1., }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = 1./60.;

    model.simulator.update(&model.bodies);
    model.simulator.gravitation(&mut model.bodies, dt);
    model.simulator.collisions(&mut model.bodies);

    color_by_acceleration(&mut model.bodies);

    let mouse_pos = app.mouse.position();
    model.offset -= mouse_pos/50. * (1./model.zoom);
}

fn mouse_wheel(_app: &App, model: &mut Model, scroll: MouseScrollDelta, _phase: TouchPhase) {
    if let MouseScrollDelta::LineDelta(_, y_delta) = scroll {
        model.zoom *= (1.5 as f32).pow(y_delta);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.zoom).xy(model.offset);

    draw.background().rgb(0.11, 0.12, 0.13);

    model.simulator.visualize(&draw, &model.bodies);

    for body in &model.bodies {
        draw.ellipse().color(body.color).xy(body.position.as_f32()).radius(body.radius as f32);
    }

    draw.to_frame(app, &frame).unwrap();
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
        body.color = rgb(pow(progress, 3)*0.2+0.8, 0.8, 0.8);
    }
}
