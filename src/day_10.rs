use std::collections::{HashSet, VecDeque};

use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add<(i64, i64)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i64, i64)) -> Pos {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

type Height = u8;

struct TopographicMap {
    map: Vec<Vec<Height>>,
    trailheads: Vec<Pos>,
}

impl TopographicMap {
    fn new(input: Input) -> Self {
        let mut trailheads = Vec::<Pos>::new();

        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.expect("valid input")
                    .char_indices()
                    .map(|(x, c)| {
                        let h = c.to_digit(10).expect("valid digit") as Height;
                        if h == 0 {
                            trailheads.push(Pos {
                                x: x as i64,
                                y: y as i64,
                            });
                        }
                        h
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { map, trailheads }
    }

    fn height_at(&self, pos: Pos) -> Option<Height> {
        let Pos { x, y } = pos;
        self.map
            .get(y as usize)
            .map_or(None, |row| row.get(x as usize))
            .copied()
    }

    fn neighbours(&self, pos: Pos) -> [Pos; 4] {
        [pos + (0, -1), pos + (0, 1), pos + (-1, 0), pos + (1, 0)]
    }

    // number of peaks (height 9) reachable from a given trailhead
    fn reachable_peaks(&self, trailhead: Pos) -> usize {
        let mut peaks = 0;

        let mut visited = HashSet::<Pos>::new();
        let mut exploring = VecDeque::<Pos>::new();

        exploring.push_back(trailhead);
        visited.insert(trailhead);

        while let Some(pos) = exploring.pop_front() {
            let h0 = self.height_at(pos).unwrap_or(0);

            for neighbour in self.neighbours(pos) {
                match self.height_at(neighbour) {
                    Some(h) if h == (h0 + 1) => {
                        if !visited.insert(neighbour) {
                            continue;
                        }
                        if h == 9 {
                            peaks += 1;
                        } else {
                            exploring.push_back(neighbour);
                        }
                    }
                    _ => {}
                }
            }
        }

        peaks
    }

    // number of distinct routes from a given trailhead to a peak
    fn trail_rating(&self, trailhead: Pos) -> usize {
        let mut distinct_routes = 0;
        let mut exploring = vec![trailhead];

        while let Some(pos) = exploring.pop() {
            let h0 = self.height_at(pos).unwrap_or(0);

            for neighbour in self.neighbours(pos) {
                match self.height_at(neighbour) {
                    Some(h) if h == (h0 + 1) => {
                        if h == 9 {
                            distinct_routes += 1;
                            continue;
                        } else {
                            exploring.push(neighbour);
                        }
                    }
                    _ => {}
                }
            }
        }

        distinct_routes
    }

    fn expolore_peaks(&self) -> usize {
        self.trailheads
            .iter()
            .map(|&th| self.reachable_peaks(th))
            .sum()
    }

    fn expolore_ratings(&self) -> usize {
        self.trailheads
            .iter()
            .map(|&th| self.trail_rating(th))
            .sum()
    }
}

#[allow(unreachable_code, unused_variables)]
pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let map = TopographicMap::new(input);

    let result = match part {
        day::Part::One => map.expolore_peaks(),
        day::Part::Two => map.expolore_ratings(),
    } as i64;

    Ok(result)
}

day_tests!("day_10-1.dat", 816, 1960);
