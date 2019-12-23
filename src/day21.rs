use intcode::*;

#[aoc_generator(day21)]
pub fn generator(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

#[aoc(day21, part1)]
pub fn part1(program: &IntcodeProgram) -> isize {
    let mut machine = Machine::new(program);

    // @ABCD
    // jump if (!A|!B|!C) && D
    let input = [
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK",
        ""
    ].join("\n");

    for c in input.chars() {
        machine.push_input(c as isize);
    }

    let mut damage_taken = 0;

    'main: loop {
        match machine.step() {
            Some(Action::RequiresInput) => unimplemented!(),
            Some(Action::Halt) => break 'main,
            Some(Action::Output(value)) => {
                if value <= 255 {
                    print!("{}", value as u8 as char);
                } else {
                    println!("Took damage: {}", value);
                    damage_taken += value;
                }
            },
            None => {}
        }
    }

    damage_taken
}


#[aoc(day21, part2)]
pub fn part2(program: &IntcodeProgram) -> isize {
    let mut machine = Machine::new(program);

    let input = [
        // (!A|!B|!C)&D &(E|H)

        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",

        "AND D J",
        
        "NOT E T",
        "NOT T T",
        "OR H T",
        "AND T J",

        "RUN",
        ""
    ].join("\n");

    for c in input.chars() {
        machine.push_input(c as isize);
    }

    let mut damage_taken = 0;

    'main: loop {
        match machine.step() {
            Some(Action::RequiresInput) => unimplemented!(),
            Some(Action::Halt) => break 'main,
            Some(Action::Output(value)) => {
                if value <= 255 {
                    print!("{}", value as u8 as char);
                } else {
                    println!("Took damage: {}", value);
                    damage_taken += value;
                }
            },
            None => {}
        }
    }

    damage_taken
}