const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const IMAGE_SIZE: usize = WIDTH*HEIGHT;

pub struct Layer {
    digits: Vec<u8>
}

impl Layer {
    pub fn at(&self, x: usize, y: usize) -> u8 {
        self.digits[y*WIDTH + x]
    }
}

#[aoc_generator(day8)]
pub fn parse_layers(input: &str) -> Vec<Layer> {
    let digits = input.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();


    let num_layers = digits.len() / IMAGE_SIZE;

    let mut layers = Vec::new();

    for i in 0..num_layers {
        layers.push(
            Layer {
                digits: digits[i*IMAGE_SIZE..i*IMAGE_SIZE+IMAGE_SIZE].to_vec(),
            }
        );
    }

    layers
}

#[aoc(day8, part1)]
pub fn check_layers(layers: &Vec<Layer>) -> usize {
    let mut fewest_zeroes = std::usize::MAX;
    let mut checksum = std::usize::MAX;

    for layer in layers {
        let mut num_zeroes = 0;
        let mut num_ones = 0;
        let mut num_twos = 0;

        for digit in layer.digits.iter() {
            match digit {
                0 => num_zeroes += 1,
                1 => num_ones += 1,
                2 => num_twos += 1,
                _ => {}
            }
        }

        if num_zeroes < fewest_zeroes {
            fewest_zeroes = num_zeroes;
            checksum = num_ones * num_twos;
        }
    }

    checksum
}

#[aoc(day8, part2)]
pub fn decode_image(layers: &Vec<Layer>) -> String {
    let mut output = String::new();
    output.push('\n');

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            for layer in layers {
                let colour = match layer.at(x, y) {
                    0 => ' ',
                    1 => '█',
                    2 => continue,
                    _ => panic!("Invalid pixel")
                };

                output.push(colour);
                break;
            }
        }

        output.push('\n');
    }

    output
}