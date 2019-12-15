use intcode::*;

#[aoc_generator(day5)]
pub fn day5_generator(input: &str) -> Vec<isize> {
    intcode_parser(input)
}

fn execute_program(program: &IntcodeProgram, inputs: &Vec<isize>) -> Vec<isize> {
    let mut memory = program.clone();
    let mut machine = Machine::new(&mut memory);
    for input in inputs {
        machine.push_input(*input);
    }
    machine.run();
    memory
}

#[test]
pub fn test_intcode() {
  pub fn execute(input: &str) -> Vec<isize> {
    execute_program(&intcode_parser(input), &vec![])
  }

//   assert_eq!(execute("1,0,0,0,99"), vec![2,0,0,0,99]);
//   assert_eq!(execute("2,3,0,3,99"), vec![2,3,0,6,99]);
//   assert_eq!(execute("2,4,4,5,99,0"), vec![2,4,4,5,99,9801]);
//   assert_eq!(execute("1,1,1,4,99,5,6,0,99"), vec![30,1,1,4,2,5,6,0,99]);
}

#[aoc(day5, part1)]
pub fn diagnostic(program: &Vec<isize>) -> String {
    let output = execute_program(&mut program.clone(), &vec![1]);

    format!("{:?}", output)
}


#[aoc(day5, part2)]
pub fn thermal(program: &Vec<isize>) -> String {
    let output = execute_program(&mut program.clone(), &vec![5]);

    format!("{:?}", output)
}

