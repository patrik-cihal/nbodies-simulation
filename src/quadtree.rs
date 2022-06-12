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

pub struct BarnesHut {
    theta: f64,
    root: Quadrant,
}

impl BarnesHut {
    pub fn new(theta: f64) -> Self {
        Self {
            theta,
            root: Quadrant::default()
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
    pub fn visualize(&self, draw: &Draw, bodies: &Vec<Body>) {
        let (center, side) = BarnesHut::compute_area(bodies);
        self.root.visualize(draw, center.as_f32(), side as f32);
    }
}

impl Simulator for BarnesHut {
    fn simulate(&mut self, bodies: &mut Vec<Body>) {
        if bodies.len() <= 1 {
            return;
        }

        self.root = Quadrant::default();

        let (center, side) = BarnesHut::compute_area(bodies);

        for (i, body) in bodies.iter().enumerate() {
            self.root.insert(BarnBody::new(i, body), center, side);
        }

        for (i, body) in bodies.iter_mut().enumerate() {
            let mut cur_side = side;
            let mut cur_level = vec![&self.root];

            while cur_level.len() > 0 {
                let mut next_level = Vec::new();
                for quadrant in cur_level {
                    if quadrant.mass < 0.1 {
                        continue;
                    }

                    let mut dir = quadrant.average_position() - body.position;
                    let distance = dir.length().max(5.0);
                    dir /= distance;

                    if cur_side / distance < self.theta || (quadrant.body.is_some() && quadrant.body.as_ref().unwrap().id != i) {
                        let force = G * body.mass * quadrant.mass / (distance*distance);
                        body.apply_force(force * dir);
                    }
                    else if quadrant.children.is_some() {
                        next_level.extend(quadrant.children.as_ref().unwrap());
                    }
                }

                cur_side /= 2.0;
                cur_level = next_level;
            }
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

    fn visualize(&self, draw: &Draw, center: Vec2, side: f32) {
        let line_color = rgb(0.17, 0.18, 0.19);

        if self.children.is_some() {
            draw.line().color(line_color).start(Vec2::new(center.x-side/2., center.y)).end(Vec2::new(center.x+side/2., center.y));
            draw.line().color(line_color).start(Vec2::new(center.x, center.y-side/2.)).end(Vec2::new(center.x, center.y+side/2.));
            for i in 0..2 {
                for j in 0..2 {
                    let new_side = side/2.;
                    let offset = Vec2::new((i as f32)-0.5, (j as f32)-0.5)*new_side;
                    self.children.as_ref().unwrap()[i+j*2].visualize(draw, center+offset, new_side);
                }
            }
        }
    }
}
