use crate::error::{Error, Result};
use crate::input::Input;
use crate::util::Vec2;
use crate::{day, day_tests};

use std::str::FromStr;

struct Robot {
    p: Vec2,
    v: Vec2,
}

impl Robot {
    fn step(&mut self, bounds: &Vec2) -> Vec2 {
        self.p.wrapping_add_mut(&self.v, bounds);
        self.p
    }
}

struct RobotInputReader {
    input: Input,
}

impl Iterator for RobotInputReader {
    type Item = Robot;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.input.read_line()?;

        let parts = line.split_whitespace();

        let mut p_and_v = parts
            .take(2)
            .map(|s| s.split('=').nth(1).map(Vec2::from_str)?.ok());

        let p = p_and_v.next().flatten().expect("valid position");
        let v = p_and_v.next().flatten().expect("valid velocity");

        Some(Robot { p, v })
    }
}

struct EbHq {
    robots: Vec<Robot>,
    bounds: Vec2,
}

impl EbHq {
    fn new(input: Input) -> Self {
        Self {
            robots: RobotInputReader { input }.collect(),
            bounds: Vec2::new(101, 103),
        }
    }

    fn qdrant(&self, p: &Vec2) -> Option<u8> {
        let mid_x = self.bounds.x / 2;
        let mid_y = self.bounds.y / 2;

        match (p.x, p.y) {
            (x, y) if x < mid_x && y < mid_y => Some(0),
            (x, y) if x > mid_x && y < mid_y => Some(1),
            (x, y) if x < mid_x && y > mid_y => Some(2),
            (x, y) if x > mid_x && y > mid_y => Some(3),
            _ => None,
        }
    }

    fn qdrant_score_after_n_seconds(&self, seconds: i64) -> u64 {
        self.robots
            .iter()
            .map(|robot| &(robot.p + &(&robot.v * seconds)) % &self.bounds)
            .map(|p| self.qdrant(&p))
            .fold(vec![0_u64; 4], |mut acc, q| {
                if let Some(q) = q {
                    acc[q as usize] += 1;
                }
                acc
            })
            .iter()
            .product::<u64>()
    }

    fn print_if_match(&self, positions: &Vec<Vec2>, pattern: &[u8]) -> bool {
        let canvas = positions.iter().fold(
            vec![vec![b'.'; self.bounds.x as usize]; self.bounds.y as usize],
            |mut acc, p| {
                acc[p.y as usize][p.x as usize] = b'#';
                acc
            },
        );

        let mut render = false;

        'outer: for row in &canvas {
            for chunk in row.chunks(pattern.len()) {
                if chunk == pattern {
                    render = true;
                    break 'outer;
                }
            }
        }

        if !render {
            return false;
        }

        for row in canvas {
            println!("{}", row.iter().map(|&c| c as char).collect::<String>());
        }

        true
    }

    // Task #1
    fn qdrant_score(&self) -> u64 {
        self.qdrant_score_after_n_seconds(100)
    }

    // Task #2 - look for a pattern that could be a Christmas tree
    fn simulate(&mut self, iterations: u64) -> Result<u64> {
        let pattern = "#######".as_bytes();

        for i in 0..iterations {
            let positions = self
                .robots
                .iter_mut()
                .map(|robot| robot.step(&self.bounds))
                .collect::<Vec<_>>();

            if self.print_if_match(&positions, pattern) {
                return Ok(i + 1);
            }
        }

        Err(Error::NoSolution(format!(
            "No Christmas tree found after {iterations} iterations",
        )))
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut ebhq = EbHq::new(input);
    let easteregg_iterations = 1000000;

    let result = match part {
        day::Part::One => ebhq.qdrant_score(),
        day::Part::Two => ebhq.simulate(easteregg_iterations)?,
    } as i64;

    Ok(result)
}

day_tests!("day_14-1.dat", 211773366, 7344);
