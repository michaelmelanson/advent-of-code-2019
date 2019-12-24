
use core::ops::RangeInclusive;
use std::collections::{BTreeSet, HashSet};

#[derive(Clone, Debug, Hash, Ord, Eq, PartialEq, PartialOrd)]
pub struct Point(u8, u8, isize);

impl Point {
    pub const fn north_edge(&self) -> bool { self.1 == 0 }
    pub const fn south_edge(&self) -> bool { self.1 == 4 }
    pub const fn west_edge(&self) -> bool { self.0 == 0 }
    pub const fn east_edge(&self) -> bool { self.0 == 4 }

    pub const fn position(&self) -> u8 {
        (self.1 * 5) + self.0 + 1
    }

    pub fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        neighbours.push(Point(self.0, self.1-1,0));
        neighbours.push(Point(self.0+1, self.1,0));
        neighbours.push(Point(self.0, self.1+1,0));
        neighbours.push(Point(self.0-1, self.1,0));
        neighbours
    }

    pub fn recursive_neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();

        // north
        if self.north_edge() {
            neighbours.push(Point(2, 1, self.2 - 1));
        } else if self.position() == 18 {
            neighbours.push(Point(0, 4, self.2 + 1));
            neighbours.push(Point(1, 4, self.2 + 1));
            neighbours.push(Point(2, 4, self.2 + 1));
            neighbours.push(Point(3, 4, self.2 + 1));
            neighbours.push(Point(4, 4, self.2 + 1));
        } else {
            neighbours.push(Point(self.0, self.1-1, self.2));
        }

        // east
        if self.east_edge() {
            neighbours.push(Point(3, 2, self.2 - 1));
        } else if self.position() == 12 {
            neighbours.push(Point(0, 0, self.2 + 1));
            neighbours.push(Point(0, 1, self.2 + 1));
            neighbours.push(Point(0, 2, self.2 + 1));
            neighbours.push(Point(0, 3, self.2 + 1));
            neighbours.push(Point(0, 4, self.2 + 1));
        } else {
            neighbours.push(Point(self.0+1, self.1, self.2));
        }

        // south
        if self.south_edge() {
            neighbours.push(Point(2, 3, self.2 - 1));
        } else if self.position() == 8 {
            neighbours.push(Point(0, 0, self.2 + 1));
            neighbours.push(Point(1, 0, self.2 + 1));
            neighbours.push(Point(2, 0, self.2 + 1));
            neighbours.push(Point(3, 0, self.2 + 1));
            neighbours.push(Point(4, 0, self.2 + 1));
        } else {
            neighbours.push(Point(self.0, self.1+1, self.2));
        }

        // west
        if self.west_edge() {
            neighbours.push(Point(1, 2, self.2 - 1));
        } else if self.position() == 14 {
            neighbours.push(Point(4, 0, self.2 + 1));
            neighbours.push(Point(4, 1, self.2 + 1));
            neighbours.push(Point(4, 2, self.2 + 1));
            neighbours.push(Point(4, 3, self.2 + 1));
            neighbours.push(Point(4, 4, self.2 + 1));
        } else {
            neighbours.push(Point(self.0-1, self.1, self.2));
        }

        neighbours
    }
}

#[test]
pub fn test_recursive_neighbours() {
    // tile 19
    let point = Point(3, 3, 0);
    assert_eq!(point.position(), 19);
    assert_eq!(point.recursive_neighbours(), vec![
        Point(3, 2, 0),
        Point(4, 3, 0),
        Point(3, 4, 0),
        Point(2, 3, 0),
    ]);

    // tile 14
    let point = Point(3, 2, 0);
    assert_eq!(point.position(), 14);
    assert_eq!(point.recursive_neighbours(), vec![
        Point(3, 1, 0),
        Point(4, 2, 0),
        Point(3, 3, 0),
        Point(4, 0, 1),
        Point(4, 1, 1),
        Point(4, 2, 1),
        Point(4, 3, 1),
        Point(4, 4, 1),
    ]);
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Grid(BTreeSet<Point>);

impl Grid {
    fn bug_present(&self, point: &Point) -> bool {
        self.0.contains(point)
    }

    fn bug_count(&self) -> usize {
        self.0.len()
    }

    fn depth_range(&self) -> RangeInclusive<isize> {
        let mut min_depth = 0;
        let mut max_depth = 0;

        for bug in self.0.iter() {
            if bug.2 < min_depth { min_depth = bug.2; }
            if bug.2 > max_depth { max_depth = bug.2; }
        }

        (min_depth-1)..=(max_depth+1)
    }

    fn neighbour_bug_count(&self, point: &Point) -> u8 {
        let mut bugs = 0;

        for neighbour in point.neighbours() {
            if self.bug_present(&neighbour) {
                bugs += 1;
            }
        }

        bugs
    }


    fn recursive_neighbour_bug_count(&self, point: &Point) -> u8 {
        let mut bugs = 0;

        for neighbour in point.recursive_neighbours() {
            if self.bug_present(&neighbour) {
                bugs += 1;
            }
        }

        bugs
    }

    pub fn advance(&self) -> Self {
        let mut new_grid = self.0.clone();

        for y in 0u8..5u8 {
            for x in 0u8..5u8 {
                let point = Point(x,y,0);
                let bug = self.bug_present(&point);
                let neighbour_count = self.neighbour_bug_count(&point);

                if bug && neighbour_count != 1 {
                    new_grid.remove(&point);
                } else if !bug && (neighbour_count == 1 || neighbour_count == 2) {
                    new_grid.insert(point);
                }
            }
        }

        Grid(new_grid)
    }


    pub fn recursive_advance(&self) -> Self {
        let mut new_grid = self.0.clone();

        for depth in self.depth_range() {
            for y in 0u8..5u8 {
                for x in 0u8..5u8 {
                    if x == 2 && y == 2 { continue; }
                    let point = Point(x,y,depth);
                    let bug = self.bug_present(&point);
                    let neighbour_count = self.recursive_neighbour_bug_count(&point);

                    if bug && neighbour_count != 1 {
                        new_grid.remove(&point);
                    } else if !bug && (neighbour_count == 1 || neighbour_count == 2) {
                        new_grid.insert(point.clone());
                    }
                }
            }
        }

        Grid(new_grid)
    }
}

#[aoc_generator(day24)]
pub fn parse_grid(input: &str) -> Grid {
    let mut bugs = BTreeSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(Point(x as u8, y as u8, 0));
            }
        }
    }

    Grid(bugs)
}

#[aoc(day24, part1)]
pub fn find_biodiversity(grid: &Grid) -> usize {
    let mut seen = HashSet::new();
    let mut grid = grid.clone();

    for _ in 0.. {
        if seen.contains(&grid) {
            break;
        }
        seen.insert(grid.clone());
        grid = grid.advance();
    }

    let mut biodiversity = 0usize;
    for y in 0u8..5u8 {
        for x in 0u8..5u8 {
            let point = Point(x,y,0);
            if grid.bug_present(&point) {
                let value = 1usize << ((y*5)+x);
                biodiversity += value;
            }
        }
    }

    biodiversity
}

#[aoc(day24, part2)]
pub fn count_recursive_bugs(grid: &Grid) -> usize {
    count_recursive_bugs_inner(grid, 200)
}

fn count_recursive_bugs_inner(grid: &Grid, time_limit: usize) -> usize {
    let mut grid = grid.clone();

    for _ in 0..time_limit {
        grid = grid.recursive_advance();
    }

    grid.bug_count()
}

#[test]
pub fn test_part2_simple_case() {
    assert_eq!(count_recursive_bugs_inner(&parse_grid("....#\n#..#.\n#.?##\n..#..\n#...."), 10), 99);
}
