use std::collections::HashMap;
use intcode::*;

#[aoc_generator(day17)]
pub fn parse_intcode(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Open,
    Scaffold
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Open => '.',
            Cell::Scaffold => '#'
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point(self.0, self.1-1),
            Point(self.0+1, self.1),
            Point(self.0, self.1+1),
            Point(self.0-1, self.1),
        ]
    }
}

fn is_intersection(map: &HashMap<Point, Cell>, point: &Point) -> bool {
    let cell = *map.get(point).unwrap_or(&Cell::Open);
    let neighbours = point.neighbours();

    cell == Cell::Scaffold && 
        neighbours.iter().all(|p| *map.get(p).unwrap_or(&Cell::Open) == Cell::Scaffold)
}

#[aoc(day17, part1)]
pub fn find_alignment_parameters(program: &IntcodeProgram) -> isize {
    let mut machine = Machine::new(program);

    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    'main: loop {
        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }

        match machine.step() {
            Some(Action::Halt) => { break 'main; },
            Some(Action::Output(value)) => {
                let cell = match value as u8 as char {
                    '\n' => { y += 1; x = 0; continue 'main; },
                    '.' => Cell::Open,
                    '#'|'^'|'>'|'v'|'<' => Cell::Scaffold,
                    c => panic!("Unknown cell character: '{}'", c)
                };

                map.insert(Point(x,y), cell);
                x += 1;
            },
            Some(Action::RequiresInput) => { unimplemented!() }
            None => {}
        }
    }

    let mut sum = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point(x, y);

            if is_intersection(&map, &point) {
                print!("O");
                sum += x*y;
            } else {
                let cell = map.get(&Point(x, y)).unwrap_or(&Cell::Open);

                print!("{}", cell.to_char());
            }
        }
        println!();
    }

    sum
}

#[aoc(day17, part2)]
pub fn collect_dust(program: &IntcodeProgram) -> isize {

    let mut machine = Machine::new(program);
    machine.write(2, &Parameter::Position(0));

    // solved by hand
    let input = [
        "A,C,A,B,A,C,B,C,B,C",
        "R,8,L,10,L,12,R,4",
        "R,8,L,10,R,8",
        "R,8,L,12,R,4,R,4",
        "n",
        ""
    ].join("\n");

    for c in input.chars() {
        machine.push_input(c as isize);
    }


    let mut dust = None;
    'main: loop {
        match machine.step() {
            Some(Action::Halt) => {
                break 'main;
            },
            Some(Action::Output(value)) => { 
                dust = Some(value);
            },
            Some(Action::RequiresInput) => unimplemented!(),
            None => {}
        }
    }

    dust.expect("Did not output amount of dust collected")
}