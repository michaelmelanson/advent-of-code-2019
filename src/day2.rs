
#[aoc_generator(day2)]
pub fn intcode_parser(input: &str) -> Vec<usize> {
  input
    .split(",")
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<usize>>()
}

pub fn execute_intcode(memory: &mut Vec<usize>) {
  let mut ip = 0;

  loop {
    let opcode = memory[ip];
    match opcode {
      1 => {
        let lhs_address = memory[ip+1];
        let rhs_address = memory[ip+2];
        let output_address = memory[ip+3];

        memory[output_address] = memory[lhs_address] + memory[rhs_address];
        ip += 4;
      },

      2 => {
        let lhs_address = memory[ip+1];
        let rhs_address = memory[ip+2];
        let output_address = memory[ip+3];

        memory[output_address] = memory[lhs_address] * memory[rhs_address];
        ip += 4;
      },

      99 => return,
      _ => panic!(format!("Unknown opcode {} at IP {}", opcode, ip))
    }

  }
}

#[aoc(day2, part1)]
pub fn check_1202(program: &Vec<usize>) -> usize {
  let mut memory = program.clone();

  memory[1] = 12;
  memory[2] = 2;

  execute_intcode(&mut memory);

  return memory[0];
}


#[aoc(day2, part2)]
pub fn execute_intcode_part2(program: &Vec<usize>) -> usize {
  for noun in 0..99 {
    for verb in 0..99 {
      let mut memory = program.clone();
      memory[1] = noun;
      memory[2] = verb;

      execute_intcode(&mut memory);

      if memory[0] == 19690720 {
        return 100 * noun + verb;
      }
    }
  }

  panic!("Did not find a solution");
}

#[test]
pub fn tests() {
  pub fn execute(input: &str) -> Vec<usize> {
    let mut program = intcode_parser(input);
    execute_intcode(&mut program);
    program
  }

  assert_eq!(execute("1,0,0,0,99"), vec![2,0,0,0,99]);
  assert_eq!(execute("2,3,0,3,99"), vec![2,3,0,6,99]);
  assert_eq!(execute("2,4,4,5,99,0"), vec![2,4,4,5,99,9801]);
  assert_eq!(execute("1,1,1,4,99,5,6,0,99"), vec![30,1,1,4,2,5,6,0,99]);
}