use std::collections::HashSet;

#[derive(Clone, Debug, Hash)]
pub struct Body {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize)
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Body> {
    let mut bodies = Vec::new();

    for line in input.lines() {
        let line = &line[1..line.len()-1];
        let parts = line.split(",").collect::<Vec<_>>();
        let x: isize = parts[0].trim()[2..].parse().unwrap();
        let y: isize = parts[1].trim()[2..].parse().unwrap();
        let z: isize = parts[2].trim()[2..].parse().unwrap();

        let body = Body {
            position: (x, y, z),
            velocity: (0, 0, 0)
        };

        bodies.push(body);
    }

    bodies
}

fn step_simulation(bodies: &mut Vec<Body>) {
    
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i <= j { continue; }

            if bodies[i].position.0 < bodies[j].position.0 {
                bodies[i].velocity.0 += 1;
                bodies[j].velocity.0 -= 1;
            } else if bodies[i].position.0 > bodies[j].position.0 {
                bodies[i].velocity.0 -= 1;
                bodies[j].velocity.0 += 1;
            }

            if bodies[i].position.1 < bodies[j].position.1 {
                bodies[i].velocity.1 += 1;
                bodies[j].velocity.1 -= 1;
            } else if bodies[i].position.1 > bodies[j].position.1 {
                bodies[i].velocity.1 -= 1;
                bodies[j].velocity.1 += 1;
            }

            if bodies[i].position.2 < bodies[j].position.2 {
                bodies[i].velocity.2 += 1;
                bodies[j].velocity.2 -= 1;
            } else if bodies[i].position.2 > bodies[j].position.2 {
                bodies[i].velocity.2 -= 1;
                bodies[j].velocity.2 += 1;
            }       
        }
    }

    for body in bodies.iter_mut() {
        body.position.0 += body.velocity.0;
        body.position.1 += body.velocity.1;
        body.position.2 += body.velocity.2;
    }
}

#[aoc(day12, part1)]
pub fn simulate_bodies(bodies: &Vec<Body>) -> isize {
    let mut bodies: Vec<Body> = bodies.clone().to_vec();

    for _ in 0..1000 {
        step_simulation(&mut bodies);
    }

    let mut energy = 0;
    for body in bodies {
        let potential_energy = 
            body.position.0.abs() + 
            body.position.1.abs() + 
            body.position.2.abs();

        let kinetic_energy = 
            body.velocity.0.abs() +
            body.velocity.1.abs() +
            body.velocity.2.abs();

        energy += potential_energy * kinetic_energy;
    }

    energy
}
