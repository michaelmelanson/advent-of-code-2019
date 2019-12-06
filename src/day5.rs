

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
