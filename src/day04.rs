
use std::cmp::max;

#[aoc_generator(day4)]
pub fn parse_range(input: &str) -> (Vec<u8>, Vec<u8>) {
  let parts = input.split("-").collect::<Vec<_>>();
  let min = parts[0].chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect::<Vec<_>>();
  let max = parts[1].chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect::<Vec<_>>();
  (min, max)
}

#[aoc(day4, part1)]
pub fn count_passwords(range: &(Vec<u8>, Vec<u8>)) -> u64 {
  count_passwords_inner(Vec::new(), range, 0, false, true, true)
}

#[aoc(day4, part2)]
pub fn count_passwords_part2(range: &(Vec<u8>, Vec<u8>)) -> u64 {
  count_passwords_inner(Vec::new(), range, 0, true, true, true)
}

pub fn count_passwords_inner(
  prior: Vec<u8>,
  range: &(Vec<u8>, Vec<u8>),
  start_index: usize,
  only_adjacent_pairs: bool,
  require_within_range_min: bool,
  require_within_range_max: bool
) -> u64 {
  if start_index == 6 {
    let mut has_double = false;

    for i in 1..6 {
      if prior[i-1] == prior[i] {

        if only_adjacent_pairs {
          if i >= 2 && prior[i-2] == prior[i-1] { continue; }
          if i < 5 && prior[i+1] == prior[i] { continue; }
        }

        has_double = true;
      }
    }

    if !has_double {
      return 0;
    }

    // println!("{} is valid", prior.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""));
    return 1; 
  }

  let mut count = 0;

  let last_digit = if start_index > 0 {
    Some(prior[start_index-1])
  } else {
    None
  };

  let min_digit = if require_within_range_min {
    max(last_digit.unwrap_or(1), range.0[start_index])
  } else {
    last_digit.unwrap_or(1)
  };

  let max_digit = if require_within_range_max {
    range.1[start_index]
  } else {
    9
  };

  for digit in min_digit..max_digit+1 {
    let mut trace = prior.clone();
    trace.push(digit);

    count += count_passwords_inner(
      trace,
      range,
      start_index + 1,
      only_adjacent_pairs,
      require_within_range_min && digit == range.0[start_index],
      require_within_range_max && digit == range.1[start_index],
    );
  }

  count
}