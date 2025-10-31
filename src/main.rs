use std::vec::Vec;

const G: f32 = 6.67408e-11; // m3 kg-1 s-2

fn main() {
    let mut bodies = vec![
        Body::new(5.0e24, [0.0, 0.0], [0.0, 0.0]), // Earth
        Body::new(7.35e22, [384400000.0, 0.0], [0.0, 1022.0]), // Moon
    ];

    let dt = 60.0; // timestep in seconds
    for _ in 0..1000 {
        let forces = super_position(&bodies);
        integral(&mut bodies, &forces, dt);
    }

    for (i, b) in bodies.iter().enumerate() {
        println!(
            "Body {} -> Position: {:?}, Velocity: {:?}",
            i, b.position, b.velocity
        );
    }
}

#[derive(Debug, Clone)]
struct Body {
    mass: f32,          // mass in kg
    position: [f32; 2], // x, y coordinates in m
    velocity: [f32; 2], // v_x, v_y velocity in m s-2
}

impl Body {
    // initializing bodies
    fn new(mass: f32, position: [f32; 2], velocity: [f32; 2]) -> Self {
        Self {
            mass,
            position,
            velocity,
        }
    }

    fn compute_gravitational_forces(&self, other_body: &Body) -> [f32; 2] {
        let dx = other_body.position[0] - self.position[0];
        let dy = other_body.position[1] - self.position[1];

        let dr2 = dx * dx + dy * dy;
        let dr = dr2.sqrt();

        let force = G * self.mass * other_body.mass / dr2;
        [force * dx / dr, force * dy / dr]
    }
}

fn super_position(bodies: &[Body]) -> Vec<[f32; 2]> {
    let mut forces = vec![[0.0, 0.0]; bodies.len()];

    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i != j {
                let f = bodies[i].compute_gravitational_forces(&bodies[j]);
                forces[i][0] += f[0];
                forces[i][1] += f[1];
            }
        }
    }

    forces
}

fn integral(bodies: &mut [Body], forces: &[[f32; 2]], dt: f32) {
    for (body, force) in bodies.iter_mut().zip(forces.iter()) {
        let accelation = [force[0] / body.mass, force[1] / body.mass];

        body.velocity[0] += accelation[0] * dt;
        body.velocity[1] += accelation[1] * dt;

        body.position[0] += body.velocity[0] * dt;
        body.position[1] += body.velocity[1] * dt;
    }
}
