use intcode::*;

const BOX_SIZE: usize = 100;

#[aoc_generator(day19)]
pub fn parse_intcode(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

fn probe_location(program: &IntcodeProgram, x: usize, y: usize) -> bool {
    let mut machine = Machine::new(program);
    machine.push_input(x as isize);
    machine.push_input(y as isize);

    'main: loop { 
        match machine.step() {
            Some(Action::RequiresInput) => unimplemented!(),
            Some(Action::Halt) => unimplemented!(),
            Some(Action::Output(value)) => {
                return value == 1;
            },
            None => {}
        }
    }
}

#[aoc(day19, part1)]
pub fn count_points(program: &IntcodeProgram) -> usize {
    let mut count: usize = 0;

    for y in 0..50 {
        for x in 0..50 {
            if probe_location(program, x, y) {
                print!("#");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }

    count
}

#[aoc(day19, part2)]
pub fn find_ship(program: &IntcodeProgram) -> usize {
    
    for y in 1000usize..10000usize {
        let mut on_column = None;
        for x in 500usize..10000usize {
            if probe_location(program, x, y) {
                on_column = Some(x);
                break;
            }
        }
        if on_column.is_none() { 
            continue;
        }
        let on_column = on_column.unwrap();

        if probe_location(program, on_column + (BOX_SIZE-1), y - (BOX_SIZE-1)) {
            return (on_column * 10000) + (y - (BOX_SIZE-1));
        }
    }

    panic!("Did not find solution");
}