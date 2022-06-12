use crate::{DVec2, PI_F64, random_f64, rgb};

pub const G: f64 = 667.43;

#[derive(Clone)]
pub struct Body {
    pub mass: f64,
    pub position: DVec2,
    pub velocity: DVec2,
    pub acceleration: DVec2,
    pub radius: f64,
    pub color: rgb::Rgb,
}

impl Body {
    fn new(mass: f64, position: DVec2, velocity: DVec2) -> Self {
        Self { mass, position, velocity, acceleration: DVec2::default(), color: rgb::Rgb::default(), radius: mass.sqrt() }
    }

    pub fn update(&mut self, force: DVec2, dt: f64) {
        self.acceleration = force / self.mass;
        self.velocity += self.acceleration*dt;
        self.position += self.velocity*dt;
    }
    
    pub fn collide(&mut self, body: &Body) {
        
    }
}

fn random_in_circle(radius: f64) -> DVec2 {
    let angle = 2.0 * PI_F64 * random_f64();
    let distance = random_f64().sqrt() * radius;
    return DVec2::new(angle.cos()*distance, angle.sin()*distance);
}

pub fn big_bang(bod_count: i32, radius: f64, expansion: f64) -> Vec<Body> {
    let mut bodies = vec![];

    for _ in 0..bod_count {
        let mass = random_f64().powf(4.)*100.+10.;
        let position = random_in_circle(radius);
        let velocity = (position + random_in_circle(radius)*0.5)*expansion;
        bodies.push(Body::new(mass, position, velocity));
    }

    return bodies;
}


pub fn stable_solar_system(bod_count: i32, radius: f64) -> Vec<Body> {
    const M: f64 = 1000.0;


    let mut bodies = vec![];
    for _ in 0..bod_count {
        let position = random_in_circle(radius);
        let distance = position.length();
        let velocity = (G*M/distance).sqrt() * DVec2::new(-position.y, position.x).normalize();
        bodies.push(Body::new(5., position, velocity));
    }

    let sun = Body::new(M, DVec2::default(), DVec2::default());
    bodies.push(sun);

    return bodies;
}
