use std::collections::HashMap;
use std::fmt::{self};

use crate::error::Result;
use crate::input::Input;
use crate::util::{Dims, Vec2};
use crate::{day, day_tests};

#[allow(unused_imports)]
use log::info;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum FenceSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct FencePiece {
    pos: Vec2,
    side: FenceSide,
}

impl FencePiece {
    fn new(pos: Vec2, side: FenceSide) -> Self {
        Self { pos, side }
    }

    fn left(pos: Vec2) -> Self {
        Self::new(pos, FenceSide::Left)
    }

    fn right(pos: Vec2) -> Self {
        Self::new(pos, FenceSide::Right)
    }

    fn top(pos: Vec2) -> Self {
        Self::new(pos, FenceSide::Top)
    }

    fn bottom(pos: Vec2) -> Self {
        Self::new(pos, FenceSide::Bottom)
    }
}

impl std::fmt::Debug for FencePiece {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let side = match self.side {
            FenceSide::Top => "T",
            FenceSide::Bottom => "B",
            FenceSide::Left => "L",
            FenceSide::Right => "R",
        };
        write!(formatter, "{}{:?}", side, self.pos)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct FenceGroupKey {
    side: FenceSide,
    axis: i64,
}

type GardenData = Vec<Vec<u8>>;
type FencesData = Vec<Vec<u8>>;
type FencePieces = Vec<FencePiece>;

type FencePieceGroups = HashMap<FenceGroupKey, FencePieces>;

struct GardenGroups {
    data: GardenData,
    fences: FencesData,

    dims: Dims,
}

impl GardenGroups {
    const BORDER_BYTE: u8 = b'_';

    fn new(mut input: Input) -> Self {
        let mut data = Vec::new();
        data.push(vec![]); // this will be replaced with a horizontal border once we know the width

        loop {
            let mut buffer = vec![Self::BORDER_BYTE];
            match input.read_line_as_bytes_into(&mut buffer) {
                Some(()) => {
                    let last_idx = buffer.len() - 1;
                    buffer[last_idx] = Self::BORDER_BYTE;
                    data.push(buffer);
                }
                None => {
                    break;
                }
            }
        }

        let width = data[data.len() - 1].len();
        let horiz_border = vec![Self::BORDER_BYTE; width];

        data[0] = horiz_border.clone();
        data.push(horiz_border);

        let height = data.len();

        let dims = Dims { width, height };

        let fences = Self::find_fence_counts(&data, dims);

        Self {
            data,
            fences,
            dims: Dims { width, height },
        }
    }

    fn alloc_data(dims: Dims) -> Vec<Vec<u8>> {
        let Dims { width, height } = dims;
        vec![vec![0; width]; height]
    }

    fn at(&self, pos: Vec2) -> u8 {
        self.data[pos]
    }

    fn num_fences_at_pos(garden: &GardenData, pos: Vec2) -> usize {
        let label = garden[pos];
        let fances = pos
            .neighbours()
            .into_iter()
            .filter(|&p| garden[p] != label)
            .count();
        fances
    }

    fn find_fence_counts(data: &GardenData, dims: Dims) -> FencesData {
        let Dims { width, height } = dims;

        let mut fences = Self::alloc_data(dims);

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let fances = Self::num_fences_at_pos(data, (x, y).into());
                fences[y][x] = fances as u8;
            }
        }

        fences
    }

    // techincally this is not a flood fill but plain old DFS... but hey, it started as with a
    // queue
    fn flood_fill_from(&self, pos: Vec2, visited: &mut Vec<Vec<u8>>, mut func: impl FnMut(&Vec2)) {
        let label = self.at(pos);
        let mut exploring = vec![pos];

        while let Some(pos) = exploring.pop() {
            if visited[pos] != 0 {
                continue;
            }

            visited[pos] = 1;

            func(&pos);

            for neighbour in pos.neighbours() {
                let nlabel = self.at(neighbour);
                if visited[neighbour] == 0 && nlabel == label {
                    exploring.push(neighbour);
                }
            }
        }
    }

    fn area_price(&self, pos: Vec2, visited: &mut Vec<Vec<u8>>) -> usize {
        let mut area = 0;
        let mut fences = 0;

        self.flood_fill_from(pos, visited, |&pos| {
            area += 1;
            fences += self.fences[pos] as usize;
        });

        area * fences
    }

    fn total_fences_price(&self) -> usize {
        let mut visited = Self::alloc_data(self.dims);

        let mut total_price = 0;

        for y in 1..self.data.len() - 1 {
            for x in 1..self.data[y].len() - 1 {
                if visited[y][x] == 0 {
                    let price = self.area_price((x, y).into(), &mut visited);
                    total_price += price;
                }
            }
        }

        total_price
    }

    fn fences_at_pos(&self, pos: Vec2, fences: &mut FencePieces) {
        let label = self.at(pos);

        for neighbour in pos.neighbours() {
            let nlabel = self.at(neighbour);
            if nlabel == label {
                continue;
            }

            let (x, y) = pos.into();
            let (nx, ny) = neighbour.into();

            if nx == x {
                if ny < y {
                    fences.push(FencePiece::top(pos));
                } else {
                    fences.push(FencePiece::bottom(pos));
                }
            } else {
                if nx < x {
                    fences.push(FencePiece::left(pos));
                } else {
                    fences.push(FencePiece::right(pos));
                }
            }
        }
    }

    fn fences_in_region(&self, pos: Vec2, visited: &mut Vec<Vec<u8>>) -> (usize, FencePieces) {
        let mut fences = Vec::new();
        let mut area = 0;

        self.flood_fill_from(pos, visited, |&pos| {
            self.fences_at_pos(pos, &mut fences);
            area += 1;
        });

        (area, fences)
    }

    fn analyze_fence_group(fences: &mut FencePieces) -> usize {
        let value = |f: &FencePiece| match f.side {
            FenceSide::Top | FenceSide::Bottom => f.pos.x,
            FenceSide::Left | FenceSide::Right => f.pos.y,
        };

        fences.sort_by_key(value);

        let runs = fences.iter().fold(Vec::new(), |mut acc, f| {
            if acc.is_empty() {
                acc.push(vec![f]);
            } else {
                let last = acc.last_mut().unwrap();
                let p = value(f);
                let p0 = value(last.last().unwrap());
                if p == p0 + 1 {
                    last.push(f);
                } else {
                    acc.push(vec![f]);
                }
            }
            acc
        });

        runs.len()
    }

    fn analyze_sides(fences: &FencePieces) -> usize {
        let mut groups = FencePieceGroups::new();
        for fence in fences {
            let key = FenceGroupKey {
                side: fence.side,
                axis: match fence.side {
                    FenceSide::Top | FenceSide::Bottom => fence.pos.y,
                    FenceSide::Left | FenceSide::Right => fence.pos.x,
                },
            };

            let group = groups.entry(key).or_insert_with(Vec::new);

            group.push(fence.clone());
        }

        groups.values_mut().map(Self::analyze_fence_group).sum()
    }

    fn total_fence_sides(&self) -> usize {
        let mut visited = Self::alloc_data(self.dims);

        let mut total_price = 0;

        for y in 1..self.data.len() - 1 {
            for x in 1..self.data[y].len() - 1 {
                if visited[y][x] == 0 {
                    let (area, fences) = self.fences_in_region((x, y).into(), &mut visited);
                    let num_sides = Self::analyze_sides(&fences);

                    total_price += num_sides * area;
                }
            }
        }

        total_price
    }
}

#[allow(unreachable_code, unused_variables)]
pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let gardens = GardenGroups::new(input);

    let result = match part {
        day::Part::One => gardens.total_fences_price(),
        day::Part::Two => gardens.total_fence_sides(),
    } as i64;

    Ok(result)
}

day_tests!("day_12-1.dat", 1549354, 937032);
