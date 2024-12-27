use crate::day;
use crate::error::Result;
use crate::input::Input;

use log::info;

use std::collections::HashSet;
use std::fmt::{self, Display};
use std::io;

type Map = Vec<Vec<u8>>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x as i32,
            y: y as i32,
        }
    }

    fn peek(&self, dir: Direction) -> Pos {
        let update_vec = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        return *self + update_vec;
    }
}

impl std::ops::Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i32, i32)) -> Pos {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

// TODO: rename to guard vector or something
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct GuardVec {
    pos: Pos,
    direction: Direction,
}

impl GuardVec {
    fn new(pos: Pos) -> GuardVec {
        GuardVec {
            pos,
            direction: Direction::Up,
        }
    }

    fn turn(&mut self) {
        self.direction = self.direction.turn();
    }

    fn walk(&mut self) {
        self.pos = self.pos.peek(self.direction);
    }
}

impl Display for GuardVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_symbol = match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", dir_symbol)
    }
}

#[derive(PartialEq)]
enum MapElement {
    Wall,
    Empty,
    OutOfBounds,
}

#[allow(dead_code)]
struct LabWalker<'a> {
    lab: &'a LabMap,
    extra_wall: (i32, i32),
}

#[allow(dead_code)]
impl<'a> LabWalker<'a> {
    fn new(lab: &'a LabMap, extra_wall: (i32, i32)) -> Self {
        LabWalker { lab, extra_wall }
    }

    fn at(&self, x: i32, y: i32) -> u8 {
        self.lab.map[y as usize][x as usize]
    }

    fn walk_or_die(&self) -> bool {
        let width = self.lab.width();
        let height = self.lab.height();

        let (mut x, mut y) = (self.lab.guard.pos.x, self.lab.guard.pos.y);
        let mut dir = 0;
        // 0 - up, 1 - right, 2 - down, 3 - left

        let mut steps = 0;
        loop {
            if (dir == 0 && y == 0)
                || (dir == 1 && x == width - 1)
                || (dir == 2 && y == height - 1)
                || (dir == 3 && x == 0)
            {
                break;
            }

            let (xx, yy) = match dir {
                0 => (x, y - 1),
                1 => (x + 1, y),
                2 => (x, y + 1),
                3 => (x - 1, y),
                _ => unreachable!(),
            };

            if self.at(xx, yy) == b'#' || (xx, yy) == self.extra_wall {
                // wall
                dir = (dir + 1) % 4;
            } else {
                x = xx;
                y = yy;
                steps += 1;
            }

            if steps > width * height {
                return true;
            }
        }

        return false;
    }
}

struct LabMap {
    map: Map,
    guard: GuardVec,
}

impl LabMap {
    // TODO: nice input reader iterator
    fn new(mut input: Input) -> Result<LabMap> {
        let mut map = Vec::new();
        let mut y = 0;
        let mut guard: Option<GuardVec> = None;

        while let Some(mut map_line) = input.read_line_as_bytes() {
            map_line.pop();

            if guard.is_none() {
                for x in 0..map_line.len() {
                    if map_line[x] == b'^' {
                        guard = Some(GuardVec::new(Pos::new(x, y)));
                    }
                }
            }

            map.push(map_line);
            y += 1;
        }

        Ok(LabMap {
            map,
            guard: guard.expect("Guard should be in the map"),
        })
    }

    fn width(&self) -> i32 {
        self.map[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.map.len() as i32
    }

    fn at(&self, pos: Pos) -> MapElement {
        match (pos.x, pos.y) {
            (x, y)
                if x < 0
                    || y < 0
                    || y >= self.map.len() as i32
                    || x >= self.map[y as usize].len() as i32 =>
            {
                MapElement::OutOfBounds
            }
            (x, y) => match self.map[y as usize][x as usize] {
                b'#' => MapElement::Wall,
                _ => MapElement::Empty,
            },
        }
    }

    fn guard_walk(&self) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.guard.pos);

        for g in GuardWalkIterator::new(self) {
            visited.insert(g.pos);
        }

        visited.len()
    }

    fn find_walls_to_cycle_guard(&self) -> usize {
        let mut path = vec![self.guard.clone()];
        {
            GuardWalkIterator::new(self).for_each(|g| path.push(g));
        }

        let mut total_possible_wall_placements = 0;
        let mut already_tested = HashSet::new();

        // for guard in &path {
        for i in 0..path.len() - 1 {
            let extra_wall = path[i + 1].pos;
            if self.at(extra_wall) == MapElement::Wall || extra_wall == self.guard.pos {
                continue;
            }

            if !already_tested.insert(extra_wall) {
                continue;
            }

            let mut walk_with_extra_all = GuardWalkIterator::new(&self)
                .with_extra_wall(extra_wall)
                .with_start(path[i].clone());

            for _ in &mut walk_with_extra_all {
                // just walk
            }

            if walk_with_extra_all.has_cycle() {
                total_possible_wall_placements += 1;
            }
        }

        total_possible_wall_placements
    }

    #[allow(dead_code)]
    fn count_walls_to_cycle_guard_simple(&self) -> usize {
        let mut total_possible_wall_placements = 0;

        for j in 0..self.map.len() {
            for i in 0..self.map[j].len() {
                let extra_wall = (i as i32, j as i32);

                let has_cycle = LabWalker::new(&self, extra_wall).walk_or_die();
                if has_cycle {
                    total_possible_wall_placements += 1;
                }
            }
        }

        total_possible_wall_placements
    }
}

struct GuardWalkIterator<'a> {
    map: &'a LabMap,
    guard: GuardVec,
    visited: HashSet<GuardVec>,

    extra_wall: Option<Pos>,
    cycle: bool,
}

impl<'a> GuardWalkIterator<'a> {
    fn new(map: &'a LabMap) -> Self {
        GuardWalkIterator {
            map,
            guard: map.guard.clone(),
            visited: HashSet::new(),
            extra_wall: None,
            cycle: false,
        }
    }

    fn with_extra_wall(mut self, pos: Pos) -> Self {
        self.extra_wall = Some(pos);
        self
    }

    fn with_start(mut self, guard: GuardVec) -> Self {
        self.guard = guard;
        self
    }

    fn has_cycle(&self) -> bool {
        self.cycle
    }

    fn at(&self, pos: Pos) -> MapElement {
        match self.extra_wall {
            Some(extra_wall) if extra_wall == pos => MapElement::Wall,
            _ => self.map.at(pos),
        }
    }
}

impl<'a> Iterator for GuardWalkIterator<'a> {
    type Item = GuardVec;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.guard.pos.peek(self.guard.direction);

        let next = self.at(next_pos);
        match next {
            MapElement::Wall => {
                self.guard.turn();
            }
            MapElement::OutOfBounds => {
                self.cycle = false;
                return None;
            }
            _ => {
                self.guard.walk();
            }
        };

        let alread_visited = !self.visited.insert(self.guard.clone());
        if alread_visited {
            self.cycle = true;
            return None;
        }

        Some(self.guard.clone())
    }
}

pub fn run(input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let lab_map = LabMap::new(input)?;

    let result = match part {
        day::Part::One => lab_map.guard_walk(),
        day::Part::Two => {
            // lab_map.count_walls_to_cycle_guard_simple();
            lab_map.find_walls_to_cycle_guard()
        }
    };

    writeln!(output, "= {}", result)?;

    info!("Day done ✅");

    Ok(())
}
