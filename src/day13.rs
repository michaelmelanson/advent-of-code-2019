use std::collections::VecDeque;

use intcode::*;

#[aoc_generator(day13)]
pub fn parse_intcode(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

#[derive(PartialEq)]
enum TileId {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball
}

struct DrawingInstruction {
  pub x: isize,
  pub y: isize,
  pub tile_id: TileId
}

#[aoc(day13, part1)]
pub fn count_blocks(program: &IntcodeProgram) -> usize {
  let mut machine = Machine::new(program);
  let mut drawing_instructions = Vec::new();
  let mut output_buffer = VecDeque::new();

  'main: loop { 
    match machine.step() {
      Some(Action::RequiresInput) => unimplemented!(),
      Some(Action::Halt) => break 'main,
      Some(Action::Output(value)) => {
        output_buffer.push_back(value);
        
        if output_buffer.len() >= 3 {
          drawing_instructions.push(DrawingInstruction {
            x: output_buffer.pop_front().unwrap(),
            y: output_buffer.pop_front().unwrap(),
            tile_id: match output_buffer.pop_front().unwrap() {
              0 => TileId::Empty,
              1 => TileId::Wall,
              2 => TileId::Block,
              3 => TileId::HorizontalPaddle,
              4 => TileId::Ball,
              _ => unimplemented!()
            }
          });
        }
      },
      None => {}
    }
  }

  let mut block_tile_count = 0;
  for inst in drawing_instructions {
    if inst.tile_id == TileId::Block {
      block_tile_count += 1;
    }
  }

  block_tile_count
}

// For part 2, run `cargo run` in the `arcade_cabinet` directory.
// Then solve it by starting automatic mode with 'l', then fast mode
// with 'f'.