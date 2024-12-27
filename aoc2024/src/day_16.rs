use std::collections::HashSet;

use crate::error::Result;
use crate::input::Input;
use crate::util::Vec2;
use crate::{day, day_tests};

enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    East,
    West,
    North,
    South,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::East => Dir::West,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
            Dir::South => Dir::North,
        }
    }

    fn as_vec(&self) -> Vec2 {
        match self {
            Dir::East => Vec2::new(1, 0),
            Dir::West => Vec2::new(-1, 0),
            Dir::North => Vec2::new(0, -1),
            Dir::South => Vec2::new(0, 1),
        }
    }
}

// impl from usize
impl From<usize> for Dir {
    fn from(i: usize) -> Self {
        match i {
            0 => Dir::East,
            1 => Dir::West,
            2 => Dir::North,
            3 => Dir::South,
            _ => panic!("Invalid direction index: {}", i),
        }
    }
}

impl<T> std::ops::Index<Dir> for [T; 4] {
    type Output = T;

    fn index(&self, dir: Dir) -> &Self::Output {
        &self[dir as usize]
    }
}

impl<T> std::ops::IndexMut<Dir> for [T; 4] {
    fn index_mut(&mut self, dir: Dir) -> &mut Self::Output {
        &mut self[dir as usize]
    }
}

type Maze = Vec<Vec<Tile>>;

struct MazeInputReader {
    input: Input,
}

impl MazeInputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn read(&mut self) -> (Maze, Vec2, Vec2) {
        let mut y = 0;
        let mut start = Vec2::new(0, 0);
        let mut end = Vec2::new(0, 0);

        let mut maze = vec![];

        while let Some(line) = self.input.read_line() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let row = line
                .char_indices()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'S' => {
                        start = (x, y).into();
                        Tile::Open
                    }
                    'E' => {
                        end = (x, y).into();
                        Tile::Open
                    }
                    _ => panic!("Unknown maze tile: {}", c),
                })
                .collect::<Vec<_>>();

            maze.push(row);

            y += 1;
        }

        (maze, start, end)
    }
}

type NodeRef = (Vec2, Dir);

#[derive(Debug)]
struct Node {
    cost: i64,
    prev: Vec<NodeRef>,
    closed: bool,
}

impl Node {
    fn empty() -> Self {
        Self {
            cost: i64::MAX,
            prev: vec![],
            closed: false,
        }
    }
}

#[derive(Debug)]
struct Cell {
    nodes: [Node; 4],
}

impl Cell {
    fn new() -> Self {
        Self {
            nodes: std::array::from_fn(|_| Node::empty()),
        }
    }

    fn cost(&self, dir: Dir) -> i64 {
        self.nodes[dir].cost
    }
}

struct MazeSolver {
    grid: Vec<Vec<Option<Cell>>>,
}

impl MazeSolver {
    fn new(maze: &Maze) -> Self {
        let grid = maze
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Wall => None,
                        Tile::Open => Some(Cell::new()),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { grid }
    }

    fn update_cost(&mut self, node_ref: &NodeRef, prev: Option<NodeRef>, new_cost: i64) {
        let &(pos, dir) = node_ref;
        let cell = self.grid[pos].as_mut().unwrap();
        let cost = cell.cost(dir);

        if new_cost < cost {
            cell.nodes[dir].cost = new_cost;
            cell.nodes[dir].prev.clear(); // reset previous nodes if we found a better path
        }

        if new_cost <= cost {
            // push previous node to the list to keep track of the path
            if let Some(prev) = prev {
                cell.nodes[dir].prev.push(prev);
            }
        }
    }

    fn min_cost_node(&mut self) -> (Vec2, Dir, &mut Node) {
        let mut min_cost = i64::MAX;
        let mut pos = Vec2::new(0, 0);
        let mut dir = Dir::East;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Some(cell) = &self.grid[y][x] {
                    for (i, node) in cell.nodes.iter().enumerate() {
                        if !node.closed && node.cost < min_cost {
                            min_cost = node.cost;
                            pos = Vec2::from((x, y));
                            dir = Dir::from(i);
                        }
                    }
                }
            }
        }

        let cell = self.grid[pos].as_mut().unwrap();
        let node = &mut cell.nodes[dir];

        (pos, dir, node)
    }

    fn shortest_path(&mut self, start: Vec2, end: Vec2) -> i64 {
        self.update_cost(&(start, Dir::East), None, 0);

        loop {
            let (pos, node_dir, node) = self.min_cost_node();
            let cost = node.cost;

            node.closed = true;

            if pos == end {
                return cost;
            }

            for dir in [Dir::East, Dir::West, Dir::North, Dir::South].iter() {
                let next_pos = pos + dir.as_vec();

                if let None = self.grid[next_pos] {
                    // wall
                    continue;
                }

                let next_cost = match dir {
                    d if *d == node_dir => 1,
                    d if *d == node_dir.opposite() => continue, // we came from there, turning back is always more expensive
                    _ => 1001,
                } + cost;

                self.update_cost(&(next_pos, *dir), Some((pos, node_dir)), next_cost);
            }
        }
    }

    fn all_shortest_paths_nodes(&mut self, end: Vec2) -> Vec<Vec2> {
        let mut backtrace = vec![];

        // push all nodes in the end cell to the backtrace stack
        self.grid[end]
            .as_ref()
            .unwrap()
            .nodes
            .iter()
            .for_each(|node| backtrace.extend(node.prev.iter()));

        let mut visited = HashSet::new();
        visited.insert(end);

        while let Some((pos, dir)) = backtrace.pop() {
            visited.insert(pos);

            let cell = self.grid[pos].as_ref().unwrap();

            backtrace.extend(cell.nodes[dir].prev.iter());
        }

        visited.into_iter().collect::<Vec<_>>()
    }

    fn reconstruct_path(&self, start: Vec2, end: Vec2) -> Vec<Vec2> {
        let path_cursor = self.grid[end]
            .as_ref()
            .unwrap()
            .nodes
            .iter()
            .find_map(|node| node.prev.first());

        let mut path = vec![end];

        if let Some(&(pos, dir)) = path_cursor {
            let mut pos = pos;
            let mut dir = dir;

            while pos != start {
                path.push(pos);

                let cell = self.grid[pos].as_ref().unwrap();
                let node = &cell.nodes[dir];

                let next = node.prev.first().unwrap();
                pos = next.0;
                dir = next.1;
            }
            path
        } else {
            vec![]
        }
    }
}

struct ReindeerMaze {
    maze: Maze,
    start: Vec2,
    end: Vec2,
    verbose: bool,
}

impl ReindeerMaze {
    fn new(input: Input, verbose: bool) -> Self {
        let (maze, start, end) = MazeInputReader::new(input).read();
        Self {
            maze,
            start,
            end,
            verbose,
        }
    }

    fn print(&self, path: &[Vec2]) {
        for (y, row) in self.maze.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let pos = Vec2::from((x, y));
                let c = if pos == self.start {
                    'S'
                } else if pos == self.end {
                    'E'
                } else if path.contains(&pos) {
                    '▓'
                } else {
                    match tile {
                        Tile::Wall => '▒',
                        Tile::Open => ' ',
                    }
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn find_shortest_path_cost(&self) -> i64 {
        let mut solver = MazeSolver::new(&self.maze);
        let cost = solver.shortest_path(self.start, self.end);

        if self.verbose {
            let path = solver.reconstruct_path(self.start, self.end);
            self.print(&path);
        }

        cost
    }

    fn find_all_shortest_paths_nodes(self) -> i64 {
        let mut solver = MazeSolver::new(&self.maze);
        let _ = solver.shortest_path(self.start, self.end);

        let nodes = solver.all_shortest_paths_nodes(self.end);
        if self.verbose {
            self.print(&nodes);
        }

        nodes.len() as i64
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let maze = ReindeerMaze::new(input, true);

    let result = match part {
        day::Part::One => maze.find_shortest_path_cost(),
        day::Part::Two => maze.find_all_shortest_paths_nodes(),
    } as i64;

    Ok(result)
}

day_tests!("day_16-1.dat", 107468, 533);
