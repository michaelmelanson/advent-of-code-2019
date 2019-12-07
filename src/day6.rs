
use std::collections::HashMap;

pub struct Orbit(String, String);

#[aoc_generator(day6)]
pub fn orbit_parser(input: &str) -> Vec<Orbit> {
    let mut orbits = Vec::new();

    for line in input.lines() {
        let parts = line.split(")").collect::<Vec<_>>();
        orbits.push(Orbit(parts[0].to_owned(), parts[1].to_owned()));
    }

    orbits
}

#[aoc(day6, part1)]
pub fn count_orbits(orbits: &Vec<Orbit>) -> usize {
    let mut orbiting: HashMap<&str, &str> = HashMap::new();
    let mut orbiters: HashMap<&str, Vec<&str>> = HashMap::new();

    for orbit in orbits {
        orbiting.insert(&orbit.1, &orbit.0);
        orbiters.entry(&orbit.0).or_default().push(&orbit.1);
    }
    
    let mut num_orbits = 0;
    let mut open_list: Vec<(&str, usize)> = Vec::new();
    open_list.push(("COM", 0));

    while let Some((object, depth)) = open_list.pop() {
        num_orbits += depth;

        if let Some(children) = orbiters.get(&object) {
            for child in children {
                open_list.push((child, depth+1));
            }
        }
    }

    num_orbits
}


#[aoc(day6, part2)]
pub fn count_transfers(orbits: &Vec<Orbit>) -> usize {
    let mut neighbours: HashMap<&str, Vec<&str>> = HashMap::new();

    for orbit in orbits {
        neighbours.entry(&orbit.0).or_default().push(&orbit.1);
        neighbours.entry(&orbit.1).or_default().push(&orbit.0);
    }
    
    let mut min_transfers = std::usize::MAX;
    let mut open_list: Vec<(&str, usize)> = Vec::new();
    let mut closed_list: Vec<&str> = Vec::new();
    
    open_list.push(("YOU", 0));

    while let Some((object, distance)) = open_list.pop() {
        closed_list.push(object);

        if *object == "SAN".to_owned() {
            if distance < min_transfers {
                min_transfers = distance - 2; // exclude each end
            }
        }

        if let Some(neighbours) = neighbours.get(&object) {
            for neighbour in neighbours {
                if !closed_list.contains(neighbour) {
                    open_list.push((neighbour, distance+1));
                }
            }
        }
    }

    min_transfers
}
