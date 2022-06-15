use nannou::Draw;
use nannou::prelude::{DVec2, Vec2};
use crate::{Body, rgb, Simulator};
use crate::universe::G;

#[derive(Clone)]
struct BarnBody {
    id: usize,
    position: DVec2,
    mass: f64,
}

impl BarnBody {
    fn new(id: usize, body: &Body) -> Self {
        Self {
            id,
            position: body.position,
            mass: body.mass
        }
    }
}

pub struct QuadTree {
    root: Quadrant,
    theta: f64,
    area: (DVec2, f64)
}

impl QuadTree {
    pub fn new(theta: f64) -> Self {
        Self {
            root: Quadrant::default(),
            theta,
            area: (DVec2::default(), 0.)
        }
    }
    pub fn compute_area(bodies: &Vec<Body>) -> (DVec2, f64) {
        let mut lt = DVec2::new(f64::INFINITY, f64::INFINITY);
        let mut rb = DVec2::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
        for body in bodies.iter() {
            lt.x = lt.x.min(body.position.x);
            lt.y = lt.y.min(body.position.y);
            rb.x = rb.x.max(body.position.x);
            rb.y = rb.y.max(body.position.y);
        }
        let center = (lt+rb)/2.0;
        let side = (rb.x-lt.x).max(rb.y-lt.y);
        return (center, side);
    }

    fn traverse<T>(&self, quadrant: &Quadrant, area: (DVec2, f64), callback: &mut T) where T: FnMut(&Quadrant, (DVec2, f64)) -> bool {
        if callback(quadrant, area) && quadrant.children.is_some() {
            for i in 0..2 {
                for j in 0..2 {
                    let new_side = area.1/2.;
                    let offset = DVec2::new((i as f64)-0.5, (j as f64)-0.5) * new_side;
                    self.traverse(&quadrant.children.as_ref().unwrap()[i+j*2], (area.0+offset, new_side), callback);
                }
            }
        }
    }
}

impl Simulator for QuadTree {
    fn visualize(&self, draw: &Draw, _bodies: &Vec<Body>) {
        let line_color = rgb(0.17, 0.18, 0.19);

        self.traverse(&self.root, self.area, &mut |quadrant: &Quadrant, (center, side): (DVec2, f64)| -> bool {
            if quadrant.children.is_some() {
                draw.line().color(line_color).start(DVec2::new(center.x-side/2., center.y).as_f32()).end(DVec2::new(center.x+side/2., center.y).as_f32());
                draw.line().color(line_color).start(DVec2::new(center.x, center.y-side/2.).as_f32()).end(DVec2::new(center.x, center.y+side/2.).as_f32());
            }
            return true;
        });
    }

    fn update(&mut self, bodies: &Vec<Body>) {
        self.root = Quadrant::default();
        self.area = QuadTree::compute_area(bodies);

        for (i, body) in bodies.iter().enumerate() {
            self.root.insert(BarnBody::new(i, body), self.area.0, self.area.1);
        }
    }

    fn collisions(&mut self, bodies: &mut Vec<Body>) {
        for i in 0..bodies.len() {
            self.traverse(&self.root, self.area, &mut |quadrant: &Quadrant, (center, side): (DVec2, f64)| -> bool {
                if  center.x - side/2. < bodies[i].position.x+bodies[i].radius && 
                    center.x + side/2. > bodies[i].position.x-bodies[i].radius && 
                    center.y - side/2. < bodies[i].position.y+bodies[i].radius &&
                    center.y + side/2. > bodies[i].position.y-bodies[i].radius
                {
                    if quadrant.body.is_some() {
                        let j = quadrant.body.as_ref().unwrap().id;
                        if i==j {
                            return false;
                        }
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
                    return true;
                }
                return false;
            });
        }
    }

    fn gravitation(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        for (i, body) in bodies.iter_mut().enumerate() {
            let mut force = DVec2::default();

            self.traverse(&self.root, self.area, &mut |quadrant: &Quadrant, (_center, side): (DVec2, f64)| {
                if quadrant.mass < 0.1 { // checks for empty quadrant
                    return false;
                }

                let offset = quadrant.average_position() - body.position;
                let distance = offset.length().max(3.);
                let dir = offset/distance;
                if side / distance < self.theta || (quadrant.body.is_some() && quadrant.body.as_ref().unwrap().id != i) { 
                    force += dir * G * body.mass * quadrant.mass / (distance*distance);
                    return false;
                }
                return true;
            });

            body.update(force, dt);
        }
    }
}

#[derive(Clone)]
struct Quadrant {
    body: Option<BarnBody>,
    position: DVec2,
    mass: f64,
    children: Option<Vec<Quadrant>>,
}

impl Default for Quadrant {
    fn default() -> Self {
        Self {
            body: None,
            position: DVec2::default(),
            mass: 0.0,
            children: None
        }
    }
}

impl Quadrant {
    fn insert(&mut self, body: BarnBody, center: DVec2, side: f64) {
        self.position += body.position*body.mass;
        self.mass += body.mass;

        if self.body.is_none() {
            if self.children.is_none() {
                self.body = Some(body);
            }
            else {
                let right = (body.position.x > center.x) as usize;
                let bottom= (body.position.y > center.y) as usize;
                let new_side = side/2.;
                let offset = DVec2::new((right as f64)-0.5, (bottom as f64)-0.5)*new_side;
                self.children.as_mut().unwrap()[right*1 + bottom*2].insert(body, center + offset, new_side);
            }
        }else {
            self.create_children();
            let cur_body = self.body.as_ref().unwrap().to_owned();
            self.body = None;
            self.insert(cur_body, center, side);
            self.insert(body, center, side);
        }
    }

    fn create_children(&mut self) {
        self.children = Some(vec![Quadrant::default(); 4]);
    }

    fn average_position(&self) -> DVec2 {
        return self.position / self.mass;
    }
}
