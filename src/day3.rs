use std::collections::{
  HashMap
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
  Right,
  Down,
  Left,
  Up
}

impl Direction {
  pub fn from(s: &str) -> Direction {
    match s {
      "R" => Direction::Right,
      "D" => Direction::Down,
      "L" => Direction::Left,
      "U" => Direction::Up,
      x => panic!("Invalid direction: {}", x)
    }
  }

  pub fn dx(&self) -> i64 {
    match self {
      Direction::Right => 1,
      Direction::Left => -1,
      _ => 0
    }
  }

  pub fn dy(&self) -> i64 {
    match self {
      Direction::Up => 1,
      Direction::Down => -1,
      _ => 0
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment(Direction, u64);

#[derive(Debug, PartialEq)]
pub struct Wire(Vec<Segment>);

#[aoc_generator(day3)]
pub fn wire_parser(input: &str) -> Vec<Wire> {
  let mut wires = Vec::new();

  for line in input.lines() {
    let mut segments = Vec::new();

    for part in line.split(",") {
      let (direction_str, length_str) = part.split_at(1);
      let direction = Direction::from(direction_str);
      let length = length_str.parse::<u64>().unwrap();

      segments.push(Segment(direction, length));
    }

    wires.push(Wire(segments));
  }

  wires
}

#[test]
pub fn test_wire_parser() {
  assert_eq!(wire_parser("R75,D30,L83,U7"), vec![
    Wire(vec![
      Segment(Direction::Right, 75),
      Segment(Direction::Down, 30),
      Segment(Direction::Left, 83),
      Segment(Direction::Up, 7)
    ])
  ]);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord, PartialEq)]
pub struct Point(i64, i64);

impl Point {
  pub fn step(&self, direction: &Direction) -> Point {
    Point(
      self.0 + direction.dx(),
      self.1 + direction.dy()
    )
  }
}

pub fn find_intersections(wires: &Vec<Wire>) -> (Vec<(Point, u64)>, HashMap<Point, u64>) {
  let mut visited = HashMap::new();
  let mut intersections = Vec::new();

  for wire in wires {
    let trace = trace_wire(wire).clone();
    
    for (point, distance) in &trace {
      if let Some(other_distance) = visited.get(point) {
        intersections.push((*point, distance + other_distance));
      }
    }

    for (point, distance) in &trace {
      visited.insert(*point, *distance);
    }
  }

  (intersections, visited)
}

#[test]
pub fn test_find_intersections() {
  {
    let (intersections, _visited) = find_intersections(
      &wire_parser("R8,U5,L5,D3\nU7,R6,D4,L4")
    );

    let mut intersections = intersections.into_iter().collect::<Vec<_>>();
    intersections.sort();

    assert_eq!(intersections, vec![
      (Point(3, 3), 40),
      (Point(6, 5), 30)
    ]);
  }

  {
    let (intersections, _visited) = find_intersections(
      &wire_parser("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
    );

    let mut intersections = intersections.into_iter().collect::<Vec<_>>();
    intersections.sort();

    // The expected output here is just the actual output and I haven't verified that it's correct.
    assert_eq!(
      intersections.into_iter().collect::<Vec<_>>(), 
      vec![
        (Point(146, 46), 624), 
        (Point(155, 4), 726),
        (Point(155, 11), 850), 
        (Point(158, -12), 610) // this is the important one
      ]);
  }
}


#[aoc(day3, part1)]
pub fn find_closest_intersection(wires: &Vec<Wire>) -> u64 {
  let (intersections, _visited) = find_intersections(wires);

  let mut closest_distance = std::u64::MAX;
  for (intersection, _) in intersections {
    let distance = intersection.0.abs() as u64 + intersection.1.abs() as u64;
    
    if distance < closest_distance {
      closest_distance = distance;
    }
  }

  closest_distance
}

#[test]
pub fn test_part1_examples() {
  assert_eq!(find_closest_intersection(
    &wire_parser("R8,U5,L5,D3\nU7,R6,D4,L4")
  ), 6);

  assert_eq!(find_closest_intersection(
    &wire_parser("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
  ), 159);

  assert_eq!(find_closest_intersection(
    &wire_parser("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
  ), 135);
}

pub fn trace_wire(wire: &Wire) -> Vec<(Point, u64)> {
  let mut trace = Vec::new();
  let mut position = Point(0, 0);
  let mut distance = 0;

  for segment in &wire.0 {
    let direction = segment.0;
    let length = segment.1;

    for _ in 0..length {
      distance += 1;
      position = position.step(&direction);
      trace.push((position, distance));
    }
  }

  trace
}

#[test]
pub fn test_trace_wire() {
  assert_eq!(trace_wire(&Wire(vec![
    Segment(Direction::Right, 3),
    Segment(Direction::Up, 2),
    Segment(Direction::Left, 5),
    Segment(Direction::Down, 3),
  ])).iter().copied().collect::<Vec<_>>(), vec![
    (Point(1, 0), 1),
    (Point(2, 0), 2),
    (Point(3, 0), 3),

    (Point(3, 1), 4),
    (Point(3, 2), 5),

    (Point(2, 2), 6),
    (Point(1, 2), 7),
    (Point(0, 2), 8),
    (Point(-1, 2), 9),
    (Point(-2, 2), 10),

    (Point(-2, 1), 11),
    (Point(-2, 0), 12),
    (Point(-2, -1), 13)
  ]);

  assert_eq!(trace_wire(&Wire(vec![
    Segment(Direction::Right, 8),
    Segment(Direction::Up, 5),
    Segment(Direction::Left, 5),
    Segment(Direction::Down, 3),
  ])).iter().copied().collect::<Vec<_>>(), vec![
    (Point(1, 0), 1),
    (Point(2, 0), 2),
    (Point(3, 0), 3),
    (Point(4, 0), 4),
    (Point(5, 0), 5),
    (Point(6, 0), 6),
    (Point(7, 0), 7),
    (Point(8, 0), 8),

    (Point(8, 1), 9),
    (Point(8, 2), 10),
    (Point(8, 3), 11),
    (Point(8, 4), 12),
    (Point(8, 5), 13),

    (Point(7, 5), 14),
    (Point(6, 5), 15),
    (Point(5, 5), 16),
    (Point(4, 5), 17),
    (Point(3, 5), 18),

    (Point(3, 4), 19),
    (Point(3, 3), 20),
    (Point(3, 2), 21)
  ]);
}

#[aoc(day3, part2)]
pub fn find_shortest_intersection(wires: &Vec<Wire>) -> u64 {
  let (intersections, _visited) = find_intersections(wires);

  let mut shortest_distance = std::u64::MAX;
  for (_intersection, distance) in intersections {  
    if distance < shortest_distance {
      shortest_distance = distance;
    }
  }

  shortest_distance
}