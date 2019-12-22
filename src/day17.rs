use std::collections::{
    HashSet,
    HashMap,
    VecDeque
};
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

    pub fn move_by(&self, direction: Direction, distance: u8) -> Point {
        match direction {
            Direction::Up => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0 + distance as isize, self.1),
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - distance as isize, self.1),
        }
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    pub fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down            
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up            
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Step {
    Left,
    Right,
    Forward(u8),
    FunctionA,
    FunctionB,
    FunctionC
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Robot(Point, Direction);

impl Robot {
    pub fn step(&self, step: &Step) -> Self {
        match step {
            Step::Left => Self(self.0, self.1.turn_left()),
            Step::Right => Self(self.0, self.1.turn_right()),  
            Step::Forward(distance) => Self(self.0.move_by(self.1, *distance), self.1),
            Step::FunctionA | Step::FunctionB | Step::FunctionC => unimplemented!()
        }
    }
}

#[derive(Debug)]
struct Path {
    latest: Robot,
    steps: Vec<Step>,
    history: HashSet<Robot>,
    visited: HashSet<Point>
}

impl Path {
    pub fn apply_step(&self, step: Step) -> Option<Path> {
        let latest = self.latest.step(&step);

        if self.history.contains(&latest) {
            return None;
        }

        let mut steps = self.steps.clone();
        steps.push(step);

        let mut history = self.history.clone();
        history.insert(latest.clone());

        let mut visited = self.visited.clone();
        visited.insert(latest.0);

        Some(Path { latest, steps, history, visited })
    }

    pub fn neighbours(&self, map: &HashMap<Point, Cell>) -> Vec<Path> {
        let mut valid_neighbours = Vec::new();
        
        if let Some(neighbour) = self.apply_step(Step::Left) {
            valid_neighbours.push(neighbour);
        }
        
        if let Some(neighbour) = self.apply_step(Step::Right) {
            valid_neighbours.push(neighbour);
        }

        if let Some(next) = self.apply_step(Step::Forward(1)) {
            if map.get(&next.latest.0) == Some(&Cell::Scaffold) {
                if is_intersection(map, &next.latest.0) || !self.visited.contains(&next.latest.0) {
                    valid_neighbours.push(next);
                }
            }
        }

        valid_neighbours
    }
}

fn extract_map(program: &IntcodeProgram) -> (HashMap<Point, Cell>, HashSet<Point>, Robot) {
    let mut machine = Machine::new(program);

    let mut scaffold_cells = HashSet::new();
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut robot = None;

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
                    '#' => Cell::Scaffold,
                    '^' => { robot = Some(Robot(Point(x,y), Direction::Up)); Cell::Scaffold },
                    '>' => { robot = Some(Robot(Point(x,y), Direction::Right)); Cell::Scaffold },
                    'v' => { robot = Some(Robot(Point(x,y), Direction::Down)); Cell::Scaffold },
                    '<' => { robot = Some(Robot(Point(x,y), Direction::Left)); Cell::Scaffold },
                    c => panic!("Unknown cell character: '{}'", c)
                };

                if cell == Cell::Scaffold {
                    scaffold_cells.insert(Point(x, y));
                }
                
                map.insert(Point(x,y), cell);
                x += 1;
            },
            Some(Action::RequiresInput) => { unimplemented!() }
            None => {}
        }
    }

    let robot = robot.expect("Did not find robot");

    (map, scaffold_cells, robot)
}

fn find_solution(map: &HashMap<Point, Cell>, initial: &Robot, scaffold_cells: &HashSet<Point>) -> Option<Vec<Step>> {
    let mut open = Vec::new();
    open.push(Path {
        latest: initial.clone(),
        steps: vec![],
        history: {
            let mut history = HashSet::new();
            history.insert(initial.clone());
            history
        },
        visited: {
            let mut visited = HashSet::new();
            visited.insert(initial.0);
            visited
        }
    });
    let mut closed = HashSet::new();

    let mut solution = None;

    while let Some(path) = open.pop() {
        closed.insert(path.latest.0);
        let leftover = scaffold_cells.len() - path.visited.len();
        if leftover == 0 {
            solution = Some(path.steps);
            break;
        }

        for neighbour in path.neighbours(&map) {
            open.push(neighbour);
        }
    }

    solution
}

fn minimize_solution(solution: &Vec<Step>) -> Vec<Step> {
    let mut solution = VecDeque::from(solution.clone());
    let mut minimized_solution = Vec::new();

    while !solution.is_empty() {
        if solution[0] == Step::Left || solution[0] == Step::Right {
            let mut turn = 0;
            let mut count = 0;

            for d in 0u8.. {
                if d as usize >= solution.len() { break; }
                if solution[d as usize] == Step::Left { turn -= 1; }
                else if solution[d as usize] == Step::Right { turn += 1; }
                else { break; }
                count += 1;
            }

            while turn >= 3 { turn -= 4; }
            while turn <= -3 { turn += 4; }

            if turn > 0 { 
                for _ in 0..turn {
                    minimized_solution.push(Step::Right);
                }
            } else {
                for _ in 0..-turn {
                    minimized_solution.push(Step::Left);
                }
            }
            for _ in 0..count { solution.pop_front(); }
        } else if solution[0] == Step::Forward(1) && solution[1] == Step::Forward(1) {
            let mut distance = None;
            for d in 2u8.. {
                if d as usize >= solution.len() { break; }
                if solution[d as usize] != Step::Forward(1) {
                    distance = Some(d);
                    break;
                }
            }
            let distance = distance.unwrap_or(9);

            minimized_solution.push(Step::Forward(distance));
            for _ in 0..distance {
                solution.pop_front();
            }
        } else {
            let step = solution.pop_front().unwrap();
            minimized_solution.push(step);
        }
    }

    minimized_solution
}

type Function = Vec<Step>;
enum MainRoutineInstruction { A, B, C }
struct MainRoutine {
    instructions: Vec<MainRoutineInstruction>,
    functions: (Function, Function, Function)
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