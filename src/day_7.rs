use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

/**
 * Generates sequences of numbers from 0 to m-1 of length n
 */
struct SeqGenerator {
    buf: Vec<u8>,
    m: u8,
    n: usize,
}

impl SeqGenerator {
    fn new(m: u8, n: u8) -> Self {
        let mut s = vec![0; n as usize];
        s[0] = u8::MAX; // this is just a trick to make the first iteration to yield [0, 0, 0, ...]

        Self {
            buf: s,
            m,
            n: n as usize,
        }
    }

    fn next(&mut self) -> Option<&[u8]> {
        let mut done = false;
        for i in 0..self.n {
            let i = i as usize;

            let a = self.buf[i].wrapping_add(1);
            if a < self.m {
                // value at i can be incremented
                self.buf[i] = a;
                break;
            }

            if i == self.n - 1 {
                // if we are at the last index, the sequence is exhausted
                done = true;
                break;
            }

            // ith index is maxed out, try next
            let b = self.buf[i + 1] + 1;
            if b < self.m {
                // value at (i+1) index can be incremented
                // reset all up to i
                for j in 0..=i {
                    self.buf[j] = 0;
                }

                self.buf[i + 1] = b;
                break;
            }

            // otherwise, continue to the next index...
        }

        if done {
            return None;
        }

        Some(&self.buf)
    }
}

const OP_MUL: u8 = 0;
const OP_ADD: u8 = 1;
const OP_CONCAT: u8 = 2;

struct BridgeEquation {
    result: i64,
    operands: Vec<i64>,
}

impl BridgeEquation {
    fn try_eval(&self, ops: &[u8]) -> Option<i64> {
        let mut result = self.operands[0] as i64;

        for (i, arg) in self.operands.iter().skip(1).enumerate() {
            let arg = *arg;

            result = match ops[i] {
                OP_MUL => result * arg,
                OP_ADD => result + arg,
                OP_CONCAT => {
                    let shift = (arg as f64).log10().floor() as u32 + 1;
                    10_i64.pow(shift as u32) * result + arg
                }
                _ => unreachable!(),
            };

            if result > self.result {
                return None;
            }
        }

        if result == self.result {
            return Some(result);
        }

        None
    }

    fn print_solution(&self, operators: &[u8]) {
        print!("{}", self.operands[0]);

        for (i, arg) in self.operands.iter().skip(1).enumerate() {
            match operators[i] {
                OP_MUL => print!(" * {}", arg),
                OP_ADD => print!(" + {}", arg),
                OP_CONCAT => print!("||{}", arg),
                _ => unreachable!(),
            }
        }

        println!(" = {}", self.result);
    }

    fn has_solution(&self, mut gen: SeqGenerator) -> bool {
        while let Some(ops) = gen.next() {
            if let Some(_) = self.try_eval(ops) {
                self.print_solution(ops);
                return true;
            }
        }

        false
    }

    fn has_solution_simple_ops(&self) -> bool {
        let gen = SeqGenerator::new(2, (self.operands.len() - 1) as u8);
        self.has_solution(gen)
    }

    fn has_solution_with_concat_op(&self) -> bool {
        let gen = SeqGenerator::new(3, (self.operands.len() - 1) as u8);
        self.has_solution(gen)
    }
}

struct RopeBridgeCalculations {
    equations: Vec<BridgeEquation>,
}

impl RopeBridgeCalculations {
    fn new(input: Input) -> Self {
        let equations = input
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(": ");

                let result = parts.next().unwrap().parse::<i64>().unwrap();

                let operands = parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|op| op.parse::<i64>().unwrap())
                    .collect();

                BridgeEquation { result, operands }
            })
            .collect();

        Self { equations }
    }

    fn find_solvable_eqs_sum<F>(&self, solver: F) -> i64
    where
        F: Fn(&BridgeEquation) -> bool,
    {
        self.equations
            .iter()
            .filter(|eq| solver(eq))
            .map(|eq| eq.result)
            .sum()
    }

    fn find_simple_solvable_eqs_sum(&self) -> i64 {
        self.find_solvable_eqs_sum(|eq| eq.has_solution_simple_ops())
    }

    fn find_solvable_with_concat_eqs_sum(&self) -> i64 {
        self.find_solvable_eqs_sum(|eq| eq.has_solution_with_concat_op())
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let bridge_calcs = RopeBridgeCalculations::new(input);

    let result = match part {
        day::Part::One => bridge_calcs.find_simple_solvable_eqs_sum(),
        day::Part::Two => bridge_calcs.find_solvable_with_concat_eqs_sum(),
    };

    Ok(result)
}

day_tests!("day_7-1.dat", 1298300076754, 248427118972289);
