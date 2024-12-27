use crate::error::{Error, Result};
use crate::input::Input;
use crate::util::Vec2;
use crate::{day, day_tests};

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum MemCell {
    Safe,
    Corrupted,
}

type Memory = Vec<Vec<MemCell>>;

struct CorruptedMemory {
    ram: Memory,
    bounds: Vec2,
    falling_bytes: Vec<Vec2>,
    falling_bytes_index: usize,
    first_wave_size: usize,
}

impl CorruptedMemory {
    fn new(input: Input, width: usize, height: usize, first_wave_size: usize) -> Self {
        let falling_bytes = input
            .lines()
            .map(|line| Vec2::from_str(line.unwrap().as_str()).unwrap())
            .collect();

        Self {
            ram: vec![vec![MemCell::Safe; width]; height],
            bounds: (width, height).into(),
            falling_bytes,
            falling_bytes_index: 0,
            first_wave_size,
        }
    }

    fn print_ram(&self) {
        for row in &self.ram {
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        MemCell::Safe => '.',
                        MemCell::Corrupted => '#',
                    }
                );
            }
            println!();
        }
    }

    fn apply_falling_bytes(&mut self, count: usize) {
        for i in 0..count {
            let pos = self.falling_bytes[i + self.falling_bytes_index];
            self.ram[pos] = MemCell::Corrupted;
        }
        self.falling_bytes_index += count;
    }

    fn find_path(&self, from: Vec2, to: Vec2) -> Option<i64> {
        let mut queue = VecDeque::from(vec![(from, 0)]);
        let mut visited = HashSet::new();

        while let Some((pos, cost)) = queue.pop_front() {
            if pos == to {
                return Some(cost);
            }

            for adj in pos.neighbours() {
                let new_pos = if adj.inside(&self.bounds) {
                    adj
                } else {
                    continue;
                };

                if self.ram[new_pos] == MemCell::Corrupted {
                    continue;
                }

                if !visited.insert(new_pos) {
                    continue;
                }

                queue.push_back((new_pos, cost + 1));
            }
        }

        None
    }

    fn find_escape_path(&mut self) -> Result<i64> {
        self.apply_falling_bytes(self.first_wave_size);
        self.print_ram();

        self.find_path(Vec2::new(0, 0), self.bounds - &Vec2::new(1, 1))
            .ok_or(Error::NoSolution(format!(
                "No path found after {} bytes fell",
                self.first_wave_size
            )))
    }

    fn find_cut_off_byte(&mut self) -> i64 {
        self.apply_falling_bytes(self.first_wave_size); // unwind first 1k, we know the path is there

        self.print_ram();

        let from = Vec2::new(0, 0);
        let to = self.bounds - &Vec2::new(1, 1);

        // this is stupid solution but the input is so small and find_path so quick it doesn't matter
        while let Some(_) = self.find_path(from, to) {
            self.apply_falling_bytes(1);
        }

        let cut_off_byte = self.falling_bytes[self.falling_bytes_index - 1];
        println!("{},{}", cut_off_byte.x, cut_off_byte.y);

        // we curently don't have a way of returing string as a day task result
        let Vec2 { x, y } = cut_off_byte;
        100 * x + y
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut mem = if let day::Part::Two = part {
        CorruptedMemory::new(input, 71, 71, 1024)
    } else {
        CorruptedMemory::new(input, 71, 71, 1024)
    };

    let result = match part {
        day::Part::One => mem.find_escape_path()?,
        day::Part::Two => mem.find_cut_off_byte(),
    } as i64;

    Ok(result)
}

day_tests!("day_18-1.dat", 246, 2250);
