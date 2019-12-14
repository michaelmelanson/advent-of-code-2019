use intcode::*;

#[aoc_generator(day7)]
pub fn day7_generator(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}


fn execute_amplifier(program: &IntcodeProgram, inputs: &Vec<isize>) -> isize {
    let mut memory = program.clone();
    let mut machine = Machine::new(&mut memory);
    for input in inputs {
        machine.push_input(*input);
    }

    loop {
        match machine.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => return output,
            Action::Halt => panic!("Halted without producing output")
        }
    }
}

fn execute_amplifier_chain(program: &IntcodeProgram, sequence: &Vec<isize>) -> isize {
    let output_a = execute_amplifier(&program, &vec![sequence[0], 0]);
    let output_b = execute_amplifier(&program, &vec![sequence[1], output_a]);
    let output_c = execute_amplifier(&program, &vec![sequence[2], output_b]);
    let output_d = execute_amplifier(&program, &vec![sequence[3], output_c]);
    let output_e = execute_amplifier(&program, &vec![sequence[4], output_d]);

    output_e
}

#[test]
fn example_program_one() {
    let program = intcode_parser("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let sequence = vec![4,3,2,1,0];

    let output = execute_amplifier_chain(&program, &sequence);

    assert_eq!(output, 43210);
}

#[aoc(day7, part1)]
pub fn find_amplifier_settings(program: &IntcodeProgram) -> isize {
    let mut best_output = std::isize::MIN;

    for one in 0..=4 {

        for two in 0..=4 {
            if two == one { continue; }

            for three in 0..=4 {
                if three == two || three == one { continue; }

                for four in 0..=4 {
                    if four == three || four == two || four == one { continue; }

                    for five in 0..=4 {
                        if five == four || five == three || five == two || five == one { continue; }

                        let sequence = vec![one, two, three, four, five];
                        let output = execute_amplifier_chain(&program, &sequence);
                        
                        if output > best_output {
                            best_output = output;
                        }
                    }
                }
            }
        }
    }

    best_output
}


fn execute_amplifier_loop(program: &IntcodeProgram, sequence: &Vec<isize>) -> isize {
    let mut amplifier_a = Machine::new(&program);
    let mut amplifier_b = Machine::new(&program);
    let mut amplifier_c = Machine::new(&program);
    let mut amplifier_d = Machine::new(&program);
    let mut amplifier_e = Machine::new(&program);

    amplifier_a.push_input(sequence[0]);
    amplifier_b.push_input(sequence[1]);
    amplifier_c.push_input(sequence[2]);
    amplifier_d.push_input(sequence[3]);
    amplifier_e.push_input(sequence[4]);

    amplifier_a.push_input(0);

    let mut thruster_input = None;

    'main: loop {
        let output_a = match amplifier_a.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => output,
            Action::Halt => break 'main
        };

        amplifier_b.push_input(output_a);
        let output_b = match amplifier_b.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => output,
            Action::Halt => break 'main
        };

        amplifier_c.push_input(output_b);
        let output_c = match amplifier_c.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => output,
            Action::Halt => break 'main
        };

        amplifier_d.push_input(output_c);
        let output_d = match amplifier_d.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => output,
            Action::Halt => break 'main
        };

        amplifier_e.push_input(output_d);
        let output_e = match amplifier_e.run() {
            Action::RequiresInput => unimplemented!(),
            Action::Output(output) => output,
            Action::Halt => break 'main
        };

        thruster_input = Some(output_e);
        amplifier_a.push_input(output_e);
    }

    thruster_input.unwrap()
}

#[aoc(day7, part2)]
pub fn find_amplifier_loop_settings(program: &IntcodeProgram) -> isize {
    let mut best_output = std::isize::MIN;

    for one in 5..=9 {

        for two in 5..=9 {
            if two == one { continue; }

            for three in 5..=9 {
                if three == two || three == one { continue; }

                for four in 5..=9 {
                    if four == three || four == two || four == one { continue; }

                    for five in 5..=9 {
                        if five == four || five == three || five == two || five == one { continue; }

                        let sequence = vec![one, two, three, four, five];
                        let output = execute_amplifier_loop(&program, &sequence);
                        
                        if output > best_output {
                            println!("New best sequence {:?} with output {}", sequence, output);

                            best_output = output;
                        }
                    }
                }
            }
        }
    }

    best_output
}