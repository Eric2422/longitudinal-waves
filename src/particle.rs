use std::collections::HashMap;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

/// Counter for the [`id`](Particle::id) property of the [`Particle`] class.
static PARTICLE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A single particle in a longitudinal wave, each connected to other particles
/// by linear springs.
struct Particle {
    id: usize,
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    acceleration: Vec<f64>,
    linked_masses: HashMap<Particle, f64>,
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("{}, {:?}", self.mass, self.position)
    }
}

impl Particle {
    /// Create a new [`Particle`] with a mass of 1.0 kg, position of (0.0, 0.0,
    /// 0.0), velocity of <0.0, 0.0, 0.0>, acceleration of <0.0, 0.0, 0.0>, and
    /// no linked masses.
    ///
    /// # Return
    ///
    /// A new [`Particle`] with a mass of 1.0 kg, position of (0.0, 0.0,
    /// 0.0), velocity of <0.0, 0.0, 0.0>, acceleration of <0.0, 0.0, 0.0>, and
    /// no linked masses.
    pub fn new() -> Particle {
        Particle {
            id: PARTICLE_COUNTER.fetch_add(1, Ordering::SeqCst),
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_masses: HashMap::new(),
        }
    }

    /// Get a new [`ParticleBuilder`].
    ///
    /// # Return
    ///
    /// A new [`ParticleBuilder`].
    pub fn builder() -> ParticleBuilder {
        ParticleBuilder::new()
    }
}


/// The builder for the [`Particle`] class.
pub struct ParticleBuilder {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    acceleration: Vec<f64>,
    linked_masses: HashMap<Particle, f64>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            mass: 1.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
            acceleration: vec![0.0, 0.0, 0.0],
            linked_masses: HashMap::new(),
        }
    }

    pub fn set_mass(mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn set_position(mut self, x: f64, y: f64, z: f64) {
        self.position = vec![x, y, z];
    }

    pub fn set_velocity(mut self, x: f64, y: f64, z: f64) {
        self.velocity = vec![x, y, z];
    }

    pub fn set_acceleration(mut self, x: f64, y: f64, z: f64) {
        self.acceleration = vec![x, y, z];
    }

    pub fn add_linked_mass(mut self, particle: Particle) {}
}
