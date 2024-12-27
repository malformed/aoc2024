use crate::error::Result;
use crate::input::Input;
use crate::util::Vec2;
use crate::{day, day_tests};

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::Write;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn as_vec(&self) -> Vec2 {
        match self {
            Dir::Left => Vec2::new(-1, 0),
            Dir::Right => Vec2::new(1, 0),
            Dir::Up => Vec2::new(0, -1),
            Dir::Down => Vec2::new(0, 1),
        }
    }
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => unreachable!(),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
            Dir::Up => write!(f, "^"),
            Dir::Down => write!(f, "v"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Box(u64),
    LargeBoxL(u64),
    LargeBoxR(u64),
}

enum Highlight {
    None,
    Red,
    Blue,
}

impl Tile {
    fn print(&self, highlight: Highlight) {
        match self {
            Tile::LargeBoxL(_) | Tile::LargeBoxR(_) => match highlight {
                Highlight::Red => print!("\x1B[1;31m{}\x1B[0m", self),
                Highlight::Blue => print!("\x1B[1;34m{}\x1B[0m", self),
                _ => print!("{}", self),
            },
            _ => print!("{}", self),
        }
    }
}

// impl display for tile
impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "▓"),
            Tile::Box(_) => write!(f, "☐"),
            Tile::LargeBoxL(_) => write!(f, "╟"),
            Tile::LargeBoxR(_) => write!(f, "╢"),
        }
    }
}

impl Tile {
    fn from_char(c: char, box_id: &mut u64) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => {
                *box_id += 1;
                Tile::Box(*box_id)
            }
            _ => Tile::Empty,
        }
    }
}

type Map = Vec<Vec<Tile>>;
type Moves = Vec<Dir>;

type MoveSet = HashMap<Vec2, Tile>;

struct WarehouseInputReader {
    input: Input,
}
impl WarehouseInputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn read_map(&mut self) -> (Vec2, Map) {
        let mut box_id = 0;
        let mut y = 0;
        let mut start = Vec2::new(0, 0);

        let mut map = vec![];

        while let Some(line) = self.input.read_line() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let row = line
                .char_indices()
                .map(|(x, c)| {
                    if (c) == '@' {
                        start = Vec2::from((x, y));
                    }
                    Tile::from_char(c, &mut box_id)
                })
                .collect::<Vec<_>>();

            map.push(row);

            y += 1;
        }

        (start, map)
    }

    fn read_moves(self) -> Moves {
        self.input
            .lines()
            .map(|line| {
                line.expect("valid line input")
                    .chars()
                    .map(Dir::from)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

struct Warehouse {
    start: Vec2,
    map: Map,
    moves: Moves,

    current_move_set: MoveSet,
    can_move: bool,
}

impl Warehouse {
    fn new(input: Input) -> Warehouse {
        let mut reader = WarehouseInputReader::new(input);

        let (start, map) = reader.read_map();
        let moves = reader.read_moves();

        Warehouse {
            start,
            map,
            moves,
            current_move_set: MoveSet::new(),
            can_move: true,
        }
    }

    fn inflate(self) -> Self {
        let inflatd_map = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Empty => [Tile::Empty, Tile::Empty],
                        Tile::Wall => [Tile::Wall, Tile::Wall],

                        Tile::Box(id) => [Tile::LargeBoxL(*id), Tile::LargeBoxR(*id)],
                        _ => unreachable!(),
                    })
                    .flatten()
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<_>>();

        Warehouse {
            start: Vec2::new(self.start.x * 2, self.start.y),
            map: inflatd_map,
            moves: self.moves,
            current_move_set: MoveSet::new(),
            can_move: true,
        }
    }

    fn render_map(&self, robot_at: Vec2) {
        print!("\x1B[2J\x1B[1;1H");

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if Vec2::from((x, y)) == robot_at {
                    print!("☺");
                    continue;
                }
                if self.current_move_set.contains_key(&Vec2::from((x, y))) {
                    if self.can_move {
                        tile.print(Highlight::Blue);
                    } else {
                        tile.print(Highlight::Red);
                    }
                } else {
                    tile.print(Highlight::None);
                }
            }
            println!();
        }
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    fn can_move_selected(&self, dir: &Vec2) -> bool {
        self.current_move_set.iter().all(|(pos, _tile)| {
            let dest = pos + dir;
            match self.map[dest] {
                Tile::Empty => true,
                Tile::Wall => false,
                Tile::LargeBoxL(_) | Tile::LargeBoxR(_)
                    if self.current_move_set.contains_key(&dest) =>
                {
                    true
                }
                _ => false,
            }
        })
    }

    fn move_selected(&mut self, dir: &Vec2) {
        for (pos, _tile) in &self.current_move_set {
            self.map[*pos] = Tile::Empty;
        }

        let mut new_current_move_set = MoveSet::new();

        for (pos, tile) in &self.current_move_set {
            let dest = pos + dir;
            self.map[dest] = *tile;
            new_current_move_set.insert(dest, *tile);
        }

        self.current_move_set = new_current_move_set;
    }

    fn step(&mut self, from: Vec2, dir: Dir) -> Vec2 {
        let dir = dir.as_vec();
        let to = from + &dir;
        let tile = self.map[to];

        match tile {
            Tile::Empty => to,
            Tile::Box(_) => {
                if let Some(dest) = self.find_empty_in_dir(&to, &dir) {
                    // move box to the empty space in direction
                    self.swap_tiles(to, dest);
                    to
                } else {
                    // stay in place
                    from
                }
            }
            Tile::LargeBoxL(_) | Tile::LargeBoxR(_) => {
                self.current_move_set = self.find_move_set(from, &dir);

                self.can_move = self.can_move_selected(&dir);
                if self.can_move {
                    self.move_selected(&dir);
                    to
                } else {
                    from
                }
            }
            _ => from, // stay in place
        }
    }

    fn find_empty_in_dir(&self, from: &Vec2, dir: &Vec2) -> Option<Vec2> {
        let width = self.map[0].len();

        for i in 1..width {
            let pos = from + &(dir * i as i64);
            match self.map[pos] {
                Tile::Empty => return Some(pos),
                Tile::Wall => return None,
                Tile::LargeBoxL(_) | Tile::LargeBoxR(_) => {
                    panic!("this can't be used to move large boxes")
                }
                _ => {}
            }
        }

        None
    }

    fn add_box_to_move_set(&self, p0: Vec2, move_set: &mut MoveSet) -> Option<(Vec2, Vec2)> {
        match self.map[p0] {
            Tile::LargeBoxL(id) => {
                let p1 = p0 + Vec2::new(1, 0);
                move_set.insert(p0, Tile::LargeBoxL(id));
                move_set.insert(p1, Tile::LargeBoxR(id));
                Some((p0, p1))
            }
            Tile::LargeBoxR(id) => {
                let p1 = p0 + Vec2::new(-1, 0);
                move_set.insert(p0, Tile::LargeBoxR(id));
                move_set.insert(p1, Tile::LargeBoxL(id));
                Some((p0, p1))
            }
            _ => None,
        }
    }

    // Find boxes that would need to move when box at pos is moved
    fn find_move_set(&self, pos: Vec2, dir: &Vec2) -> MoveSet {
        let mut visiting = vec![pos];
        let mut move_set = MoveSet::new();

        while let Some(pos) = visiting.pop() {
            let next = pos + dir;
            if move_set.contains_key(&next) {
                continue;
            }
            if let Some((p0, p1)) = self.add_box_to_move_set(next, &mut move_set) {
                visiting.push(p0);
                visiting.push(p1);
            }
        }

        move_set
    }

    fn swap_tiles(&mut self, a: Vec2, b: Vec2) {
        let aux = self.map[a];
        self.map[a] = self.map[b];
        self.map[b] = aux;
    }

    fn gps(&self) -> usize {
        let mut acc = 0;

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match self.map[y][x] {
                    Tile::Box(_) | Tile::LargeBoxL(_) => acc += 100 * y + x,
                    _ => {}
                }
            }
        }

        acc
    }

    fn replay_moves(&mut self, render: bool) -> usize {
        let mut pos = self.start;

        for mi in 0..self.moves.len() {
            let m = self.moves[mi];

            pos = self.step(pos, m);

            if render {
                self.render_map(pos);
            }
        }

        self.gps()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut warehouse = Warehouse::new(input);

    let result = match part {
        day::Part::One => warehouse.replay_moves(false),
        day::Part::Two => {
            let mut warehouse = warehouse.inflate();
            warehouse.replay_moves(false)
        }
    } as i64;

    Ok(result)
}

day_tests!("day_15-1.dat", 1495147, 1524905);
