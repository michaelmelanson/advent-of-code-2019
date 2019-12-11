use std::collections::HashSet;

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: isize, y: isize) -> isize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[derive(Debug, PartialEq)]
pub struct Asteroid { x: isize, y: isize }

impl Asteroid {
    pub fn direction_to(&self, other: &Asteroid) -> (isize, isize) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let divisor = gcd(dx, dy).abs();

        (dx / divisor, dy / divisor)
    }
}

#[aoc_generator(day10)]
pub fn parse_map(input: &str) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, position) in line.chars().enumerate() {
            if position == '#' {
                asteroids.push(Asteroid { x: x as isize, y: y as isize });
            }
        }
    }
    
    asteroids
}

#[aoc(day10, part1)]
pub fn find_best_location(asteroids: &Vec<Asteroid>) -> usize {
    let mut best_count = 0;

    for asteroid in asteroids {
        let count = count_visible_asteroids(&asteroid, asteroids);

        if count > best_count {
            best_count = count;
        }
    }

    best_count
}

pub fn count_visible_asteroids(station: &Asteroid, asteroids: &Vec<Asteroid>) -> usize {
    let mut directions = HashSet::new();

    for asteroid in asteroids {
        if asteroid != station {
            let direction = station.direction_to(asteroid);
            directions.insert(direction);
        }
    }

    directions.len()
}

#[test]
fn simple_test_one() {
    assert_eq!(
        find_best_location(
            &parse_map(".#..#\n.....\n#####\n....#\n...##")
        ), 
        8
    );
}

#[test]
pub fn simple_test_two() {
    assert_eq!(
        find_best_location(
            &parse_map("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")
        ), 
        33
    );
}