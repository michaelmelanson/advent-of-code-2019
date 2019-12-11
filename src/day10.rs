use std::collections::{HashSet, HashMap};

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
    pub fn distance_to(&self, other: &Asteroid) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;

        (dx*dx + dy*dy).sqrt()
    }
}

#[derive(Clone, Eq, Hash, Debug, PartialEq)]
pub struct Direction(isize, isize);

impl Direction {
    pub fn angle(&self) -> f64 {
        let mut angle = (self.1 as f64).atan2(self.0 as f64) - (std::f64::consts::PI / 2.0);

        while angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }

        angle.to_degrees()
    }
}

#[test]
fn test_direction_angle() {
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    assert_delta!(Direction(0, 1).angle(), 0.0, 0.00001);
    assert_delta!(Direction(1, 0).angle(), std::f64::consts::PI, 0.00001);
}


impl std::cmp::PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_angle = self.angle();
        let other_angle = other.angle();

        if self_angle.is_nan() || other_angle.is_nan() {
            None
        } else if self_angle > other_angle { 
            Some(std::cmp::Ordering::Greater)
        } else if self_angle < other_angle {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::cmp::Ord for Direction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}


impl Asteroid {
    pub fn direction_to(&self, other: &Asteroid) -> Direction {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let divisor = gcd(dx, dy).abs();

        Direction(dx / divisor, dy / divisor)
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
pub fn find_best_count(asteroids: &Vec<Asteroid>) -> usize {
    let (_, count) = find_best_station(asteroids);
    count
}

fn find_best_station(asteroids: &Vec<Asteroid>) -> (&Asteroid, usize) {
    let mut best_station = None;
    let mut best_count = 0;

    for asteroid in asteroids {
        let count = count_visible_asteroids(&asteroid, asteroids);

        if count > best_count {
            best_count = count;
            best_station = Some(asteroid);
        }
    }

    (best_station.unwrap(), best_count)
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
        find_best_station(
            &parse_map(".#..#\n.....\n#####\n....#\n...##")
        ), 
        (&Asteroid { x: 3, y: 4 }, 8)
    );
}

#[test]
pub fn simple_test_two() {
    assert_eq!(
        find_best_station(
            &parse_map("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")
        ), 
        (&Asteroid { x: 5, y: 8 }, 33)
    );
}

#[aoc(day10, part2)]
pub fn find_200th_blasted_asteroid(asteroids: &Vec<Asteroid>) -> isize {
    let (station, _)  = find_best_station(asteroids);
    println!("Blasting from {:?}", station);
    let mut blasting_list: HashMap<Direction, Vec<&Asteroid>> = HashMap::new();

    for asteroid in asteroids {
        if station == asteroid { continue; } // don't blast ourselves

        let direction = station.direction_to(asteroid);
        blasting_list.entry(direction).or_default().push(asteroid);
    }

    for asteroids in blasting_list.values_mut() {
        asteroids.sort_by_cached_key(|a| (-station.distance_to(a) * 1000.0).round() as isize);
    }

    let directions = {
        let mut directions = blasting_list.keys().cloned().collect::<Vec<_>>();
        directions.sort();
        directions
    };

    let mut blasted = 0;
    for direction in directions.iter().cycle() {
        let asteroids = blasting_list.get_mut(direction).unwrap();

        if asteroids.len() > 0 {
            let asteroid = asteroids.pop().unwrap();
            blasted += 1;
            println!("Blasted asteroid #{} at {:?} (angle {:.0}, distance {:.0})", 
                blasted, 
                asteroid, 
                station.direction_to(asteroid).angle(),
                station.distance_to(asteroid)
            );

            if blasted == 200 {
                return asteroid.x * 100 + asteroid.y;
            }
        }
    }

    panic!("No solution")
}
