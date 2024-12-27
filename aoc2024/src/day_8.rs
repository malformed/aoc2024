use crate::day;
use crate::error::Result;
use crate::input::Input;

use log::info;

use std::collections::HashSet;
use std::io;

// Generates subsets of size of a set with length n
struct SubsetGenerator {
    m: usize,
    n: usize,
    indices: Vec<usize>, // subset indices to the set we choose from
}

impl SubsetGenerator {
    fn new(m: u8, n: usize) -> Self {
        let mut indices = (0..m as usize).collect::<Vec<usize>>();

        indices.last_mut().map(|x| *x -= 1); // this is a trick so that first call to next yields the initial configuration

        Self {
            m: m as usize,
            n,
            indices,
        }
    }

    fn next(&mut self) -> Option<&[usize]> {
        // indices ...[a, b, c, ...] pointers to the original set,

        // 1) find index such that it can be incremented

        let mut done = true;

        for k in (0..self.m).rev() {
            let a = self.indices[k] + 1;

            // max value for the index is that of at (k + 1) or N
            let max = self.indices.get(k + 1).map_or(self.n, |x| *x);

            if a < max {
                // we found an index to bump
                self.indices[k] = a;

                // reset all above k
                let mut reset_val = a + 1;
                for j in k + 1..self.m as usize {
                    self.indices[j] = reset_val;
                    reset_val += 1;
                }

                done = false;
                break;
            }
        }

        if done {
            return None;
        };

        Some(&self.indices)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn make_vector(a: &Vec2, b: &Vec2) -> Self {
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }

    fn inv(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    fn in_bounds(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

struct AntinodeIterator {
    pos: Vec2,
    dir: Vec2,
    width: i32,
    height: i32,
}

impl AntinodeIterator {
    fn new(start: Vec2, dir: Vec2, map: &CityAntennaMap) -> Self {
        Self {
            pos: start,
            dir,
            width: map.width,
            height: map.height,
        }
    }
}

impl Iterator for AntinodeIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos;

        if !next.in_bounds(self.width, self.height) {
            return None;
        }

        self.pos = self.pos + self.dir;
        Some(next)
    }
}

type Antennas = Vec<Vec2>;

struct CityAntennaMap {
    antennas_at_freq: Vec<Antennas>,
    width: i32,
    height: i32,
}

impl CityAntennaMap {
    fn new(input: Input) -> Self {
        let mut antennas_at_freq = vec![Antennas::new(); 128];

        let mut width = 0_usize;
        let mut height = 0_usize;

        input.lines().enumerate().for_each(|(y, line)| {
            let line = line.expect("valid input");

            height = y + 1;
            width = line.len();

            line.char_indices().for_each(|(x, c)| match c {
                '.' => {}
                freq => {
                    antennas_at_freq
                        .get_mut(freq as usize)
                        .expect("input antennas are lower ASCII symbols")
                        .push(Vec2::from((x, y)));
                }
            });
        });

        Self {
            antennas_at_freq,
            width: width as i32,
            height: height as i32,
        }
    }

    fn adjacent_antinodes(&self, a: &Vec2, b: &Vec2) -> impl Iterator<Item = Vec2> {
        let ab = Vec2::make_vector(a, b);

        let a_antinode = AntinodeIterator::new(*a, ab.inv(), self).skip(1).take(1);
        let b_antinode = AntinodeIterator::new(*b, ab, self).skip(1).take(1);

        a_antinode.chain(b_antinode)
    }

    fn all_antinodes(&self, a: &Vec2, b: &Vec2) -> impl Iterator<Item = Vec2> {
        let ab = Vec2::make_vector(a, b);

        let a_antinodes = AntinodeIterator::new(*a, ab.inv(), self);
        let b_antinodes = AntinodeIterator::new(*b, ab, self);

        a_antinodes.chain(b_antinodes)
    }

    fn find_antinodes_for_freq(&self, antennas: &Antennas, all: bool) -> Vec<Vec2> {
        let mut antinodes = Vec::new();

        if antennas.len() < 2 {
            return antinodes;
        }

        let mut pairs_gen = SubsetGenerator::new(2, antennas.len());

        while let Some(pair) = pairs_gen.next() {
            match pair {
                [a, b] => {
                    if all {
                        antinodes.extend(self.all_antinodes(&antennas[*a], &antennas[*b]));
                    } else {
                        antinodes.extend(self.adjacent_antinodes(&antennas[*a], &antennas[*b]));
                    };
                }
                _ => unreachable!(),
            }
        }

        antinodes
    }

    fn find_antinodes(&self, all: bool) -> usize {
        self.antennas_at_freq
            .iter()
            .map(|antennas| self.find_antinodes_for_freq(antennas, all))
            .flatten()
            .collect::<HashSet<Vec2>>()
            .len()
    }

    fn find_adjecent_antinodes(&self) -> usize {
        self.find_antinodes(false)
    }

    fn find_all_antinodes(&self) -> usize {
        self.find_antinodes(true)
    }
}

#[allow(unreachable_code, unused_variables, unused_mut)]
pub fn run(input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let city_antenna_map = CityAntennaMap::new(input);

    let result = match part {
        day::Part::One => city_antenna_map.find_adjecent_antinodes(),
        day::Part::Two => city_antenna_map.find_all_antinodes(),
    };

    writeln!(output, "= {result}")?;

    info!("Day done ✅");
    Ok(())
}
