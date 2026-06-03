struct Particle {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    acceleration: Vec<f64>,
    linked_masses: Vec<Particle>,
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("{}, {:?}", self.mass, self.position)
    }
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_masses: vec![],
        }
    }
}
