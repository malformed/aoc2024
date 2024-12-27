use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use crate::error::Result;
use crate::input::Input;
use crate::util::grid::Grid;
use crate::util::Vec2;
use crate::{day, day_tests};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Keypad {
    Key(u8),
    None,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "↑",
                Self::Down => "↓",
                Self::Left => "←",
                Self::Right => "→",
                Self::A => "A",
            }
        )
    }
}

impl Dir {
    fn to_int(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
            Self::A => 4,
        }
    }
}

impl<T> std::ops::Index<Dir> for Vec<T> {
    type Output = T;

    fn index(&self, index: Dir) -> &Self::Output {
        &self[index.to_int()]
    }
}

impl<T> std::ops::IndexMut<Dir> for Vec<T> {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        &mut self[index.to_int()]
    }
}

impl Into<usize> for Dir {
    fn into(self) -> usize {
        self.to_int()
    }
}

type KeypadGrid = Grid<Vec2>;
type DirTable = Grid<Vec<Dir>>;

struct KeypadTable {
    keypad: Grid<Keypad>,
    keypad_table: KeypadGrid,

    dir_table: DirTable,
    cache: HashMap<(Dir, Dir, u8), u64>,
}

impl KeypadTable {
    fn new() -> Self {
        let keypad = Grid::new(vec![
            vec![Keypad::Key(7), Keypad::Key(8), Keypad::Key(9)],
            vec![Keypad::Key(4), Keypad::Key(5), Keypad::Key(6)],
            vec![Keypad::Key(1), Keypad::Key(2), Keypad::Key(3)],
            vec![Keypad::None, Keypad::Key(0), Keypad::Key(10)],
        ]);

        Self {
            keypad,
            keypad_table: Grid::with_size(11u8, 11u8, Vec2::new(0, 0)),
            dir_table: Self::init_handmade_dir_table(),
            cache: HashMap::new(),
        }
        .init_keypad_table()
    }

    fn init_keypad_table(mut self) -> Self {
        for from in 0..=0xa {
            for to in 0..=0xa {
                let from_pos = self.key_pos(Keypad::Key(from));
                let to_pos = self.key_pos(Keypad::Key(to));

                self.keypad_table[(from, to)] = to_pos - &from_pos;
            }
        }
        self
    }

    fn key_pos(&self, key: Keypad) -> Vec2 {
        for (pos, k) in self.keypad.iter() {
            if *k == key {
                return pos;
            }
        }
        unreachable!("all keys are present in the keypad");
    }

    fn init_handmade_dir_table() -> DirTable {
        let mut table: Grid<Vec<Dir>> = Grid::with_size(5u8, 5u8, vec![]);

        table[(Dir::A, Dir::Up)] = vec![Dir::Left];
        table[(Dir::A, Dir::Left)] = vec![Dir::Down, Dir::Left, Dir::Left];
        table[(Dir::A, Dir::Right)] = vec![Dir::Down];
        table[(Dir::A, Dir::Down)] = vec![Dir::Left, Dir::Down];

        table[(Dir::Up, Dir::A)] = vec![Dir::Right];
        table[(Dir::Up, Dir::Left)] = vec![Dir::Down, Dir::Left];
        table[(Dir::Up, Dir::Right)] = vec![Dir::Down, Dir::Right];
        table[(Dir::Up, Dir::Down)] = vec![Dir::Down];

        table[(Dir::Left, Dir::A)] = vec![Dir::Right, Dir::Right, Dir::Up];
        table[(Dir::Left, Dir::Up)] = vec![Dir::Right, Dir::Up];
        table[(Dir::Left, Dir::Right)] = vec![Dir::Right, Dir::Right];
        table[(Dir::Left, Dir::Down)] = vec![Dir::Right];

        table[(Dir::Right, Dir::A)] = vec![Dir::Up];
        table[(Dir::Right, Dir::Up)] = vec![Dir::Left, Dir::Up];
        table[(Dir::Right, Dir::Left)] = vec![Dir::Left, Dir::Left];
        table[(Dir::Right, Dir::Down)] = vec![Dir::Left];

        table[(Dir::Down, Dir::A)] = vec![Dir::Up, Dir::Right];
        table[(Dir::Down, Dir::Up)] = vec![Dir::Up];
        table[(Dir::Down, Dir::Left)] = vec![Dir::Left];
        table[(Dir::Down, Dir::Right)] = vec![Dir::Right];

        for i in 0..table.height() {
            for j in 0..table.width() {
                table[(i, j)].push(Dir::A);
            }
        }

        table
    }

    fn push_n_arrows(v: &mut Vec<Dir>, arrow: Dir, n: i64) {
        for _ in 0..n {
            v.push(arrow);
        }
    }

    fn move_vertical(v: &mut Vec<Dir>, pos: Vec2) {
        Self::push_n_arrows(v, if pos.y > 0 { Dir::Down } else { Dir::Up }, pos.y.abs());
    }

    fn move_horizontal(v: &mut Vec<Dir>, pos: Vec2) {
        Self::push_n_arrows(
            v,
            if pos.x > 0 { Dir::Right } else { Dir::Left },
            pos.x.abs(),
        );
    }

    fn moves_at_level(&mut self, path: &Vec<Dir>, depth: u8) -> u64 {
        self.cache.clear();

        let mut total = 0;
        let mut prev = Dir::A;
        for dir in path {
            let moves = self.count_moves(prev, *dir, depth - 1);
            prev = *dir;
            total += moves;
        }
        total
    }

    fn keypad_path(
        &mut self,
        at: Keypad,
        remaining: &[Keypad],
        path: &mut Vec<Dir>,
        depth: u8,
    ) -> u64 {
        if remaining.is_empty() {
            let cost = self.moves_at_level(path, depth);
            return cost;
        }

        let (first, tail) = remaining.split_first().expect("non-empty remaining slice");

        if let (Keypad::Key(from), Keypad::Key(to)) = (at, *first) {
            let path_vec = self.keypad_table[(from, to)];

            let from_pos = self.key_pos(at);

            // try moving vertically and horizonally first while avoiding Empty space in the keypad, and use the best path
            let c1 = if from_pos.x == 0 && from_pos.y + path_vec.y == 3 {
                u64::MAX
            } else {
                let mut path1 = path.clone();
                Self::move_vertical(&mut path1, path_vec);
                Self::move_horizontal(&mut path1, path_vec);
                path1.push(Dir::A);
                self.keypad_path(*first, tail, &mut path1, depth)
            };

            let c2 = if from_pos.y == 3 && from_pos.x + path_vec.x == 0 {
                u64::MAX
            } else {
                let mut path2 = path.clone();
                Self::move_horizontal(&mut path2, path_vec);
                Self::move_vertical(&mut path2, path_vec);
                path2.push(Dir::A);
                self.keypad_path(*first, tail, &mut path2, depth)
            };

            c1.min(c2)
        } else {
            unreachable!("only Keypad::Key is expected");
        }
    }

    fn count_moves(&mut self, from: Dir, to: Dir, level: u8) -> u64 {
        let path = &self.dir_table[(from, to)];

        if level == 0 {
            return path.len() as u64;
        }

        let mut total = 0;
        let mut prev = Dir::A;

        // TODO: don't clone here
        for dir in path.clone() {
            let moves = if let Some(x) = self.cache.get(&(prev, dir, level - 1)) {
                *x
            } else {
                self.count_moves(prev, dir, level - 1)
            };
            total += moves;
            prev = dir;
        }

        self.cache.insert((from, to, level), total);

        total
    }
}

struct KeypadConundrum {
    keypad: KeypadTable,
    kecodes: Vec<String>,
}

impl KeypadConundrum {
    pub fn new(input: Input) -> Self {
        Self {
            keypad: KeypadTable::new(),
            kecodes: input.lines().map(|l| l.unwrap()).collect(),
        }
    }

    fn decode_keycode(&self, keycode: &str) -> Vec<Keypad> {
        keycode
            .chars()
            .map(|c| {
                let x = c.to_digit(16).expect("valid input keycode") as u8;
                Keypad::Key(x)
            })
            .collect()
    }

    fn moves_for_keycode(&mut self, keycode: &str, depth: u8) -> u64 {
        let keycode_value = keycode[0..keycode.len() - 1]
            .parse::<u64>()
            .expect("numeric keycode");

        let keycode = self.decode_keycode(keycode);

        let mut path = Vec::new();
        let cost = self
            .keypad
            .keypad_path(Keypad::Key(0xA), &keycode, &mut path, depth);

        println!("{cost} * {keycode_value} = {}", cost * keycode_value);

        keycode_value * cost
    }

    fn count_moves(&mut self, depth: u8) -> u64 {
        self.kecodes
            .clone()
            .iter()
            .map(|keycode| self.moves_for_keycode(keycode, depth))
            .sum()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut keypad = KeypadConundrum::new(input);

    let result = match part {
        day::Part::One => keypad.count_moves(2),
        day::Part::Two => keypad.count_moves(25),
    } as i64;

    Ok(result)
}

day_tests!("day_21-1.dat", 231564, 281212077733592);
