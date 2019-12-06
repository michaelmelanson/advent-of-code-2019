

#[aoc_generator(day5)]
pub fn intcode_parser(input: &str) -> Vec<isize> {
  input
    .split(",")
    .map(|s| s.parse::<isize>().unwrap())
    .collect::<Vec<isize>>()
}

#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(isize)
}

impl Parameter {
    pub fn new(mode: isize, value: isize) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            _ => unimplemented!()
        }
    }

    pub fn resolve(&self, memory: &Vec<isize>) -> isize {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(position) => memory[*position]
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt
}

enum Action {
    Output(isize),
    Halt
}

struct Machine<'a> {
    memory: &'a mut Vec<isize>,
    ip: usize,
    inputs: &'a Vec<isize>,
    input_index: usize
}

impl <'a> Machine<'a> {
    pub fn new(memory: &'a mut Vec<isize>, inputs: &'a Vec<isize>) -> Self {
        Machine {
            memory, 
            ip: 0,
            inputs,
            input_index: 0
        }
    }

    fn read(&mut self) -> isize {
        let value = self.memory[self.ip];
        self.ip += 1;
        value
    }

    fn jump(&mut self, address: usize) {
        self.ip = address;
    }

    fn read_input(&mut self) -> isize {
        let value = self.inputs[self.input_index];
        self.input_index += 1;
        value
    }

    fn write(&mut self, value: isize, parameter: &Parameter) {
        match parameter {
            Parameter::Position(position) => {
                self.memory[*position] = value;
            },
            Parameter::Immediate(_) => unimplemented!()
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        let instruction_value = self.read();
        let opcode      = instruction_value % 100;
        let first_mode  = (instruction_value / 100) % 10;
        let second_mode = (instruction_value / 1000) % 10;
        let third_mode  = (instruction_value / 10000) % 10;

        match opcode {
            1 => Instruction::Add(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            2 => Instruction::Multiply(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            3 => Instruction::Input(
                Parameter::new(first_mode, self.read())
            ),

            4 => Instruction::Output(
                Parameter::new(first_mode, self.read())
            ),

            5 => Instruction::JumpIfTrue(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read())
            ),

            6 => Instruction::JumpIfFalse(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read())
            ),

            7 => Instruction::LessThan(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read())
            ),

            8 => Instruction::Equals(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read())
            ),

            99 => Instruction::Halt,
            _ => unimplemented!()
        }
    }

    pub fn step(&mut self) -> (Instruction, Option<Action>) {
        let instruction = self.next_instruction();
        let mut action = None;

        match &instruction {
            Instruction::Add(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory);
                let rhs = rhs.resolve(&self.memory);
                self.write(lhs + rhs, &output);
            },

            Instruction::Multiply(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory);
                let rhs = rhs.resolve(&self.memory);
                self.write(lhs * rhs, &output);
            },

            Instruction::Input(output) => {
                let value = self.read_input();
                self.write(value, output);
            },

            Instruction::Output(value) => {
                action = Some(Action::Output(value.resolve(&self.memory)));
            }

            Instruction::JumpIfTrue(value, target) => {
                let value = value.resolve(&self.memory);

                if value != 0 {
                    let target = target.resolve(&self.memory) as usize;
                    self.jump(target);
                }
            },

            Instruction::JumpIfFalse(value, target) => {
                let value = value.resolve(&self.memory);

                if value == 0 {
                    let target = target.resolve(&self.memory) as usize;
                    self.jump(target);
                }
            },

            Instruction::LessThan(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory);
                let rhs = rhs.resolve(&self.memory);

                if lhs < rhs {
                    self.write(1, output);
                } else {
                    self.write(0, output);
                }
            },

            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory);
                let rhs = rhs.resolve(&self.memory);

                if lhs == rhs {
                    self.write(1, output);
                } else {
                    self.write(0, output);
                }
            },


            Instruction::Halt => {
                action = Some(Action::Halt)
            },
        }

        (instruction, action)
    }
}

pub fn execute_program(memory: &mut Vec<isize>, input: &Vec<isize>) -> Vec<isize> {
    let mut machine = Machine::new(memory, &input);
    let mut output = Vec::new();
    
    'main: loop {
        let (instruction, action) = machine.step();
        println!("Instruction: {:?}", instruction);

        match action {
            Some(Action::Halt) => {
                println!("Halting");
                break 'main;
            },

            Some(Action::Output(value)) => {
                println!("Output: {}", value);
                output.push(value);
            },

            None => {}
        }
    }

    output
}


#[test]
pub fn test_intcode() {
  pub fn execute(input: &str) -> Vec<isize> {
    let mut program = intcode_parser(input);
    execute_program(&mut program, &vec![]);
    program
  }

  assert_eq!(execute("1,0,0,0,99"), vec![2,0,0,0,99]);
  assert_eq!(execute("2,3,0,3,99"), vec![2,3,0,6,99]);
  assert_eq!(execute("2,4,4,5,99,0"), vec![2,4,4,5,99,9801]);
  assert_eq!(execute("1,1,1,4,99,5,6,0,99"), vec![30,1,1,4,2,5,6,0,99]);
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

