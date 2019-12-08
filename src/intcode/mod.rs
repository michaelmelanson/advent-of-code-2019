
pub type IntcodeProgram = Vec<isize>;
pub type IntcodeMemory = Vec<isize>;

pub fn intcode_parser(input: &str) -> IntcodeProgram {
    input.split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<IntcodeProgram>()
}

#[derive(Debug)]
pub enum Parameter {
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

    pub fn resolve(&self, memory: &IntcodeMemory) -> isize {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(position) => memory[*position]
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
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

pub enum Action {
    Output(isize),
    Halt
}

pub struct Machine {
    memory: IntcodeMemory,
    ip: usize,
    inputs: Vec<isize>,
}

impl Machine {
    pub fn new(memory: &IntcodeMemory) -> Self {
        Machine {
            memory: memory.clone(), 
            ip: 0,
            inputs: Vec::new(),
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
        self.inputs.remove(0)
    }

    pub fn push_input(&mut self, input: isize) {
        self.inputs.push(input);
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

    pub fn step(&mut self) -> Option<Action> {
        let mut action = None;

        let instruction = self.next_instruction();
        
        match instruction {
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
                self.write(value, &output);
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
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            },

            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory);
                let rhs = rhs.resolve(&self.memory);

                if lhs == rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            },


            Instruction::Halt => {
                action = Some(Action::Halt)
            },
        }

        action
    }


    pub fn run(&mut self) -> Action {
        loop {
            let action = self.step();

            if let Some(action) = action {
                return action;
            }
        }
    }
}
