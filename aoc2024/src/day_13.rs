use crate::error::Result;
use crate::input::Input;
use crate::util::math::checked_int_div;
use crate::util::Vec2;
use crate::{day, day_tests};

#[allow(unused_imports)]
use log::info;

struct ClawMachinesReader {
    input: Input,
    eof: bool,
}

impl ClawMachinesReader {
    fn new(input: Input) -> ClawMachinesReader {
        ClawMachinesReader { input, eof: false }
    }
}

impl ClawMachinesReader {
    fn read_vec2(&mut self, is_button: bool) -> Option<Vec2> {
        let parts = self
            .input
            .read_line()
            .expect("button input")
            .split(':') // -> Button N: X+m, Y=+n
            .skip(1) // skip the label
            .take(1)
            .map(|s| {
                s.trim()
                    .split(", ") // -> "X+m", "Y=+n"
                    .map(|s| {
                        s.to_string()
                            .split(if is_button { '+' } else { '=' })
                            .skip(1) // skip the X or Y
                            .next()
                            .expect("offset or position")
                            .parse::<i64>()
                            .expect("a number")
                    })
                    .collect::<Vec<_>>()
            })
            .next()
            .expect("valid input");

        if let &[x, y] = parts.as_slice() {
            Some(Vec2 { x, y })
        } else {
            None
        }
    }
}

impl std::iter::Iterator for ClawMachinesReader {
    type Item = ClawMachine;

    fn next(&mut self) -> Option<ClawMachine> {
        if self.eof {
            return None;
        }

        let button_a = self.read_vec2(true).expect("A button input");
        let button_b = self.read_vec2(true).expect("B button input");
        let prize = self.read_vec2(false).expect("prize input");

        println!("A: {:?}, B: {:?}, P: {:?}", button_a, button_b, prize);

        self.eof = self.input.read_line().is_none(); // consume the empty line and mark the end

        Some(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}

struct ClawMachine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl ClawMachine {
    /*
    * ... or, when the comment is larger than the actual solution... ;)
    *
    * Reaching the prize can be expressed as a linear combination of application of buttons A and B.
    * Let A and B be vectors representing the two buttons movements and P be the prize.

    * The goal is to find a and b such that Aa + Bb = P where a and b are integers.
    * This is a simple system of linear equations.
    * Let's expand the matrix/vector notation and solve:

    * a * Ax + b * Bx = Px (1)
    * a * Ay + b * By = Py (2)

    * # from (1) we get:
    * b = (Px - a * Ax) / Bx (3.1)

    * # and from (2:
    * b = (Py - a * Ay) / By (3.2)

    * # subtract the two equations:
    * (Px - a * Ax) / Bx = (Py - a * Ay) / By (4)

    * # solve for a:
    * a = (Py * Bx - Px * By) / (Ay * Bx - Ax * By) (5)

    * # substitute a back into (3.1) or (3.2) to get b
    * ...

    * We know that the puzle input doesn't contain degenerate cases, A, B are always linearly independent. So we just deal with the cases where there is at most one solution.
    */
    fn solve_with_offset(&self, offset: &Vec2) -> Option<Vec2> {
        let Vec2 { x: px, y: py } = self.prize + offset;

        let Vec2 { x: ax, y: ay } = self.button_a;
        let Vec2 { x: bx, y: by } = self.button_b;

        checked_int_div(py * bx - px * by, ay * bx - ax * by)
            .and_then(|a| checked_int_div(px - a * ax, bx).map(|b| Vec2::new(a, b)))
    }
}

struct Arcade {
    claw_machines: Vec<ClawMachine>,
}

impl Arcade {
    fn new(input: Input) -> Arcade {
        Arcade {
            claw_machines: ClawMachinesReader::new(input).collect(),
        }
    }

    fn solve_with_offset(&self, offset: &Vec2) -> i64 {
        let a_cost = 3;
        let b_cost = 1;

        self.claw_machines
            .iter()
            .map(|m| match ClawMachine::solve_with_offset(m, offset) {
                Some(ab) => ab.x * a_cost + ab.y * b_cost,
                None => 0,
            })
            .sum()
    }

    fn solve(&self) -> i64 {
        self.solve_with_offset(&(0, 0).into())
    }

    fn solve_large(&self) -> i64 {
        let offset = 10000000000000_usize;
        self.solve_with_offset(&(offset, offset).into())
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let arcade = Arcade::new(input);

    let result = match part {
        day::Part::One => arcade.solve(),
        day::Part::Two => arcade.solve_large(),
    } as i64;

    Ok(result)
}

day_tests!("day_13-1.dat", 32026, 89013607072065);
