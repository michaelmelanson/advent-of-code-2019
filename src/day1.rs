use math::round::floor;
use std::vec::Vec;

#[aoc_generator(day1)]
pub fn mass_parser(input: &str) -> Vec<i64> {
    input.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn fuel_for_mass(mass: i64) -> i64 {
    let fuel = (floor(mass as f64 / 3., 0) - 2.) as i64;
    if fuel < 0 {
        return 0;
    }

    fuel + fuel_for_mass(fuel)
}

#[aoc(day1, part1)]
pub fn day1_part1(masses: &Vec<i64>) -> i64 {
    let mut fuel: i64 = 0;

    for mass in masses {
        fuel += (floor(*mass as f64 / 3., 0) - 2.) as i64;
    }
    
    fuel
}

#[aoc(day1, part2)]
pub fn day1_part2(masses: &Vec<i64>) -> i64 {
    let mut fuel: i64 = 0;

    for mass in masses {
        fuel += fuel_for_mass(*mass);
    }
    
    fuel
}