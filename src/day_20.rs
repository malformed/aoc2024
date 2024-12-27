use std::collections::HashSet;

use crate::error::Result;
use crate::input::Input;
use crate::util::grid::Grid;
use crate::util::Vec2;
use crate::{day, day_tests};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Path(i64), // value is a distance from start
}

type Track = Grid<Tile>;

struct MazeInputReader {
    input: Input,
}

impl MazeInputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn read(&mut self) -> (Vec<Vec<Tile>>, Vec2, Vec2) {
        let mut y = 0;
        let mut start = Vec2::new(0, 0);
        let mut end = Vec2::new(0, 0);

        let mut track = vec![];

        while let Some(line) = self.input.read_line() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let row = line
                .char_indices()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Path(0),
                    'S' => {
                        start = (x, y).into();
                        Tile::Path(0)
                    }
                    'E' => {
                        end = (x, y).into();
                        Tile::Path(0)
                    }
                    _ => panic!("Unknown maze tile: {}", c),
                })
                .collect::<Vec<_>>();

            track.push(row);

            y += 1;
        }

        (track, start, end)
    }
}

struct RaceTrack {
    track: Track,
    start: Vec2,
    end: Vec2,
}

impl RaceTrack {
    fn from_input(input: Input) -> Self {
        let mut reader = MazeInputReader::new(input);
        let (track, start, end) = reader.read();

        let mut inst = Self {
            track: Grid::new(track),
            start,
            end,
        };

        inst.label_path();
        inst
    }

    fn label_path(&mut self) {
        let mut distance = 0;
        let mut p = self.start;

        self.track[self.start] = Tile::Path(i64::MAX);

        while p != self.end {
            distance += 1;
            for n in p.neighbours() {
                match self.track[n] {
                    Tile::Wall => continue,
                    Tile::Path(0) => {
                        self.track[n] = Tile::Path(distance);
                        p = n;
                        break;
                    }
                    Tile::Path(d) if d > 0 => continue,
                    _ => unreachable!(),
                }
            }
        }
        self.track[self.start] = Tile::Path(0);
    }

    fn at(&self, pos: Vec2) -> Option<Tile> {
        if pos.inside(&self.track.dims()) {
            Some(self.track[pos])
        } else {
            None
        }
    }

    fn for_each_in_manhattan_circle(center: Vec2, radius: i64, mut f: impl FnMut(Vec2)) {
        for y in -radius..=radius {
            for x in -radius..=radius {
                let p = Vec2::new(x, y);
                if p.manhattan_len() <= radius {
                    f(center + p);
                }
            }
        }
    }

    fn find_cheats(&self, threshold: i64, radius: i64) -> usize {
        let mut used_cheats = HashSet::new(); // (p0, p1) | cheat start and end positions

        for (p0, tile) in self.track.iter() {
            if let Tile::Path(d0) = tile {
                Self::for_each_in_manhattan_circle(p0, radius, |p1| {
                    if let Some(Tile::Path(d1)) = self.at(p1) {
                        let shortcut = d1 - d0 - &p1.manhattan_dist(&p0);

                        if shortcut >= threshold {
                            used_cheats.insert((p0, p1));
                        }
                    }
                });
            }
        }

        used_cheats.len()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let race_track = RaceTrack::from_input(input);

    let result = match part {
        day::Part::One => race_track.find_cheats(100, 2),
        day::Part::Two => race_track.find_cheats(100, 20),
    } as i64;

    Ok(result)
}

day_tests!("day_20-1.dat", 1406, 1006101);
