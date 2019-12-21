use std::collections::{HashMap, VecDeque};

pub type PortalName = [char;2];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point(usize, usize);

impl Point {
    fn neighbours(&self) -> Vec<Self> {
        vec![
            Point(self.0,   self.1-1),
            Point(self.0+1, self.1),
            Point(self.0,   self.1+1),
            Point(self.0-1, self.1),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PointWithDepth(Point, usize);

impl PointWithDepth {
    fn neighbours(&self) -> Vec<Self> {
        self.0.neighbours().iter()
            .map(|p| PointWithDepth(*p, self.1))
            .collect::<Vec<_>>()
    }

    fn from_portal_destination(portal: &Portal, base_depth: usize) -> Option<Self> {
        let depth = match portal.edge {
            Edge::Inside => base_depth + 1,
            Edge::Outside => base_depth - 1
        };

        if depth > 0 {
            Some(PointWithDepth(portal.to, depth))
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Portal { to: Point, name: PortalName, edge: Edge }

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Edge {
    Inside,
    Outside
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Cell {
    Wall,
    Open,
    Portal(Portal) 
}

#[derive(Debug)]
pub struct Maze {
    cells: HashMap<Point, Cell>,
    start: Point,
    end: Point
}

#[aoc_generator(day20)]
pub fn parse_maze(input: &str) -> Maze {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut cells = HashMap::new();
    let mut labels = HashMap::new();

    let mut min_x = std::usize::MAX;
    let mut min_y = std::usize::MAX;
    let mut max_x = std::usize::MIN;
    let mut max_y = std::usize::MIN;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                labels.insert(Point(x,y), c);
            } else { 
                let cell = match c {
                    '#' => Some(Cell::Wall),
                    '.' => Some(Cell::Open),
                    _ => None
                };

                if let Some(cell) = cell {
                    cells.insert(Point(x, y), cell);
                    
                    if x < min_x { min_x = x; }
                    if y < min_y { min_y = y; }
                    if x > max_x { max_x = x; }
                    if y > max_y { max_y = y; }
                }
            }
        }
    }

    let maze_thickness = 33;
    let min_inside_x = min_x + maze_thickness - 1;
    let min_inside_y = min_y + maze_thickness - 1;
    let max_inside_x = max_x - maze_thickness + 1;
    let max_inside_y = max_y - maze_thickness + 1;

    let mut portal_ends = HashMap::new();

    fn add_portal(cells: &mut HashMap<Point, Cell>, portal_ends: &mut HashMap<PortalName, (Point, Edge)>, name: &PortalName, point: &Point, edge: Edge) {
        if let Some((other_end, other_edge)) = portal_ends.get(name) {
            cells.insert(*point, Cell::Portal(Portal { to: *other_end, name: name.clone(), edge: edge }));
            cells.insert(*other_end, Cell::Portal(Portal { to: *point, name: name.clone(), edge: *other_edge }));
        } else {
            portal_ends.insert(name.clone(), (*point, edge));
        }
    }

    // top outside edge
    for x in min_x..=max_x {
        let y = min_y;
        let point = Point(x,y);

        if let Some(Cell::Open) = cells.get(&point) {
            let name = [
                *labels.get(&Point(x, y-2)).unwrap(),
                *labels.get(&Point(x, y-1)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Outside);
        }
    }

    // right outside edge
    for y in min_y..=max_y {
        let x = max_x;
        let point = Point(x,y);

        if let Some(Cell::Open) = cells.get(&point) {
            let name = [
                *labels.get(&Point(x+1, y)).unwrap(),
                *labels.get(&Point(x+2, y)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Outside);
        }
    }

    // bottom outside edge
    for x in min_x..=max_x {
        let y = max_y;
        let point = Point(x,y);

        if let Some(Cell::Open) = cells.get(&point) {
            let name = [
                *labels.get(&Point(x, y+1)).unwrap(),
                *labels.get(&Point(x, y+2)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Outside);
        }
    }


    // left outside edge
    for y in min_y..=max_y {
        let x = min_y;
        let point = Point(x,y);

        if let Some(Cell::Open) = cells.get(&point) {
            let name = [
                *labels.get(&Point(x-2, y)).unwrap(),
                *labels.get(&Point(x-1, y)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Outside);
        }
    }

    // top inside edge
    for x in min_inside_x..=max_inside_x {
        let y = min_inside_y;
        let point = Point(x,y);
        let cell = cells.get(&point);

        if let Some(Cell::Open) = cell {
            let name = [
                *labels.get(&Point(x, y+1)).unwrap(),
                *labels.get(&Point(x, y+2)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Inside);
        }
    }

    // right inside edge
    for y in min_inside_y..=max_inside_y {
        let x = max_inside_x;
        let point = Point(x,y);
        let cell = cells.get(&point);

        if let Some(Cell::Open) = cell {
            let name = [
                *labels.get(&Point(x-2, y)).unwrap(),
                *labels.get(&Point(x-1, y)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Inside);
        }
    }

    // bottom inside edge
    for x in min_inside_x..=max_inside_x {
        let y = max_inside_y;
        let point = Point(x,y);
        let cell = cells.get(&point);

        if let Some(Cell::Open) = cell {
            let name = [
                *labels.get(&Point(x, y-2)).unwrap(),
                *labels.get(&Point(x, y-1)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Inside);
        }
    }

    // left inside edge
    for y in min_inside_y..=max_inside_y {
        let x = min_inside_x;
        let point = Point(x,y);
        let cell = cells.get(&point);

        if let Some(Cell::Open) = cell {
            let name = [
                *labels.get(&Point(x+1, y)).unwrap(),
                *labels.get(&Point(x+2, y)).unwrap()
            ];
            add_portal(&mut cells, &mut portal_ends, &name, &point, Edge::Inside);
        }
    }

    let (start, _) = *portal_ends.get(&['A', 'A']).expect("Could not find start position");
    let (end, _) = *portal_ends.get(&['Z', 'Z']).expect("Could not find end position");

    Maze {
        cells, start, end
    }
}

#[aoc(day20, part1)]
pub fn find_path(maze: &Maze) -> usize {
    let mut shortest_solution = std::usize::MAX;
    let mut open = Vec::new();
    open.push(vec![maze.start]);

    while let Some(candidate) = open.pop() {
        let head = candidate.last().unwrap();
        let mut neighbours = head.neighbours();
        if let Some(Cell::Portal(portal)) = maze.cells.get(head) {
            neighbours.push(portal.to);
        }

        for neighbour in neighbours {
            if neighbour == maze.end {
                let path_length = candidate.len();
                if path_length < shortest_solution {
                    shortest_solution = path_length;
                }
                continue;
            } 
            
            if candidate.contains(&neighbour) {
                continue;
            }

            let navigable = match maze.cells.get(&neighbour) {
                Some(Cell::Open) => true,
                Some(Cell::Portal(_)) => true,
                _ => false
            };

            if !navigable { 
                continue; 
            }
            
            let mut new_candidate = candidate.clone();
            new_candidate.push(neighbour);
            open.push(new_candidate);
        }
    }

    shortest_solution
}

#[aoc(day20, part2)]
pub fn find_path_recursive(maze: &Maze) -> usize {
    let mut open = VecDeque::new();
    open.push_back(vec![PointWithDepth(maze.start, 1)]);

    while let Some(candidate) = open.pop_front() {
        let head = candidate.last().unwrap();
        let mut neighbours = head.neighbours();
        if let Some(Cell::Portal(portal)) = maze.cells.get(&head.0) {
            if let Some(destination) = PointWithDepth::from_portal_destination(portal, head.1) {
                if destination.1 <= 50 {
                    neighbours.push(destination);
                }
            }
        }

        for neighbour in neighbours {
            if neighbour == PointWithDepth(maze.end, 1) {
                // cheat -- turns out that the first one we come to is the shortest 🤷‍♂️
                return candidate.len();
            }

            if candidate.contains(&neighbour) {
                continue;
            }

            let navigable = match maze.cells.get(&neighbour.0) {
                Some(Cell::Open) => true,
                Some(Cell::Portal(_)) => true,
                _ => false
            };

            if !navigable { 
                continue; 
            }
            
            let mut new_candidate = candidate.clone();
            new_candidate.push(neighbour);
            open.push_front(new_candidate);
        }
    }

    panic!("Did not find solution!");
}
