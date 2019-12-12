use std::collections::HashMap;
use crate::intcode::*;

#[derive(Debug)]
enum Turn {
    Left,
    Right
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn dx(&self) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0
        }
    }

    pub fn dy(&self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0
        }
    }

    pub fn turn(&self, turn: &Turn) -> Direction {
        match (self, turn) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Position(isize, isize);

impl Position {
    pub fn move_direction(&self, direction: &Direction) -> Position {
        Position(
            self.0 + direction.dx(), 
            self.1 + direction.dy()
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Colour {
    Black,
    White
}

#[aoc_generator(day11)]
pub fn parse_intcode(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

fn run_painting_robot(program: &IntcodeProgram, paint: &mut HashMap<Position, Colour>) {
    let mut machine = Machine::new(program);

    let mut position = Position(0,0);
    let mut direction = Direction::Up;

    'main: loop {
        let colour;
        loop {
            match machine.step() {
                Some(Action::RequiresInput) => {
                    machine.push_input(match paint.get(&position).unwrap_or(&Colour::Black) {
                        Colour::Black => 0,
                        Colour::White => 1
                    });
                },
                Some(Action::Halt) => break 'main,
                Some(Action::Output(value)) => {
                    colour = match value {
                        0 => Colour::Black,
                        1 => Colour::White,
                        _ => unimplemented!()
                    };
                    break;
                },
                None => {}
            }
        }

        let turn;
        loop {
            match machine.step() {
                Some(Action::RequiresInput) => {
                    machine.push_input(match paint.get(&position).unwrap_or(&Colour::Black) {
                        Colour::Black => 0,
                        Colour::White => 1
                    });
                },
                Some(Action::Halt) => break 'main,
                Some(Action::Output(value)) => {
                    turn = match value {
                        0 => Turn::Left,
                        1 => Turn::Right,
                        _ => unimplemented!()
                    };
                    break;
                },
                None => {}
            }
        }

        paint.insert(position, colour);
        direction = direction.turn(&turn);
        position = position.move_direction(&direction);
    }
}

#[aoc(day11, part1)]
pub fn count_panels(program: &IntcodeProgram) -> usize {
    let mut ship_paint = HashMap::new();

    run_painting_robot(program, &mut ship_paint);

    ship_paint.len()
}

#[aoc(day11, part2)]
pub fn draw_label(program: &IntcodeProgram) -> String{
    let mut ship_paint = HashMap::new();
    ship_paint.insert(Position(0, 0), Colour::White);

    run_painting_robot(program, &mut ship_paint);

    let mut output = String::new();
    output.push_str("\n");
    for y in (-5..=0).rev() {
        for x in 0..40 {
            output.push_str(match ship_paint.get(&Position(x, y)).unwrap_or(&Colour::Black) {
                Colour::Black => " ",
                Colour::White => "█"
            });
        }
        output.push_str("\n");
    }

    output
}

