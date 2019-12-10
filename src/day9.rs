use crate::intcode::*;

#[aoc_generator(day9)]
pub fn day9_generator(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

#[aoc(day9, part1)]
pub fn boost(input: &IntcodeProgram) -> isize {
    let mut machine = Machine::new(input);
    machine.push_input(1);

    let mut output = Vec::new();
    
    'main: loop { 
        match machine.run() {
            Action::Output(value) => output.push(value),
            Action::Halt => break 'main
        }
    }

    output[0]
}


#[aoc(day9, part2)]
pub fn sensor_boost(input: &IntcodeProgram) -> isize {
    let mut machine = Machine::new(input);
    machine.push_input(2);

    let mut output = Vec::new();
    
    'main: loop { 
        match machine.run() {
            Action::Output(value) => output.push(value),
            Action::Halt => break 'main
        }
    }

    output[0]
}