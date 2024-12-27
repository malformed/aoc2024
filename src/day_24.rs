use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

use log::info;

type WireLabel = [u8; 3];

type Values = HashMap<WireLabel, bool>;
type Wires = HashSet<WireLabel>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy)]
struct Gate {
    left: WireLabel,
    right: WireLabel,
    output: WireLabel,
    op: GateType,
}

fn print_label(label: &WireLabel) -> String {
    format!("{}", String::from_utf8_lossy(&label[..]))
}

fn parse_label(label: &str) -> WireLabel {
    label
        .as_bytes()
        .try_into()
        .expect("wire label is 3 characters")
}

fn label_from_u8(prefix: &str, label: u8) -> WireLabel {
    let label = format!("{}{:02}", prefix, label);
    parse_label(&label)
}

impl Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " [{:?}({:?},{:?}) -> {:?}] ",
            self.op,
            print_label(&self.left),
            print_label(&self.right),
            print_label(&self.output)
        )
    }
}

struct InputReader {
    input: Input,
}

impl InputReader {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn read_input_wires(&mut self) -> Values {
        let mut values = HashMap::new();
        while let Some(line) = self.input.read_line() {
            if line == "\n" {
                break;
            }
            let mut parts = line.split(": ");

            let label = parts.next().expect("wire label");
            let value = parts.next().expect("wire value").trim_end();

            let value = match value {
                "1" => true,
                "0" => false,
                _ => unreachable!("boolean value"),
            };

            values.insert(parse_label(label), value);
        }
        values
    }

    fn read_gates(self) -> (Vec<Gate>, Wires) {
        let mut output_wires = Wires::new();
        let gates = self
            .input
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.split(" ");

                let left = parse_label(parts.next().expect("input #1"));
                let gate = parts.next().expect("gate");
                let right = parse_label(parts.next().expect("input #2"));
                let output = parse_label(parts.skip(1).next().expect("output"));

                if output[0] == b'z' && !output_wires.contains(&output) {
                    output_wires.insert(output);
                }

                Gate {
                    left,
                    right,
                    output,
                    op: match gate {
                        "AND" => GateType::And,
                        "OR" => GateType::Or,
                        "XOR" => GateType::Xor,
                        _ => unimplemented!("unknown gate type"),
                    },
                }
            })
            .collect();
        (gates, output_wires)
    }
}

#[derive(Debug)]
struct WiringError {
    gate: Option<Gate>,
    expected_op: GateType,
}

enum CircuitResult {
    Valid,
    Invalid(WiringError),
    NotFound,
}

struct CrossedWires {
    values: Values,
    gates: Vec<Gate>,
    output_wires: Wires,

    original_values: Values,
}

impl CrossedWires {
    fn from_input(input: Input) -> Self {
        let mut reader = InputReader::new(input);
        let values = reader.read_input_wires();
        let (gates, output_wires) = reader.read_gates();

        Self {
            original_values: values.clone(),
            values,
            gates,
            output_wires,
        }
    }

    fn find_gates(&self, filter: impl Fn(&Gate) -> bool) -> Vec<Gate> {
        self.gates
            .iter()
            .filter(|g| filter(*g))
            .map(|gate| *gate)
            .collect()
    }

    fn try_find_gate(&self, output: WireLabel) -> Option<Gate> {
        self.gates
            .iter()
            .find(|&gate| gate.output == output)
            .map(|gate| *gate)
    }

    fn gate_with_output(&self, output: WireLabel) -> Gate {
        self.try_find_gate(output).expect("gate with output")
    }

    fn decode_variable(&self, prefix: &str) -> u64 {
        let result = (0..)
            .into_iter()
            .map_while(|i| {
                let wire = format!("{}{:02}", prefix, i);

                self.values.get(&parse_label(&wire)).map(|&value| value)
            })
            .fold((0, 1), |(acc, mask), value| {
                let acc = acc + mask * value as u64;
                let mask = mask << 1;
                (acc, mask)
            });
        result.0
    }

    fn eval(&mut self) {
        let output_gates = self.find_gates(|gate| self.output_wires.contains(&gate.output));

        let mut stack = output_gates;

        while let Some(gate) = stack.last() {
            let left_val = self.values.get(&gate.left);
            let right_val = self.values.get(&gate.right);

            match (left_val, right_val) {
                (Some(left), Some(right)) => {
                    let result = match gate.op {
                        GateType::And => left & right,
                        GateType::Or => left | right,
                        GateType::Xor => left ^ right,
                    };

                    self.values.insert(gate.output, result);
                    stack.pop();
                }
                (None, Some(_)) => {
                    stack.push(self.gate_with_output(gate.left));
                }
                (Some(_), None) => {
                    stack.push(self.gate_with_output(gate.right));
                }
                (None, None) => {
                    stack.extend([
                        self.gate_with_output(gate.left),
                        self.gate_with_output(gate.right),
                    ]);
                }
            }
        }
    }

    fn reset_values(&mut self) {
        self.values = self.original_values.clone();
        for (_, value) in &mut self.values {
            *value = false;
        }
    }

    // Task #1
    fn find_z_value(&mut self) -> u64 {
        self.eval();
        self.decode_variable("z")
    }

    fn rewire(&mut self, swaps: &[(WireLabel, WireLabel)]) {
        for (from, to) in swaps {
            self.gates.iter_mut().for_each(|gate| {
                if gate.output == *from {
                    gate.output = *to;
                } else if gate.output == *to {
                    gate.output = *from;
                }
            });
        }
    }

    // checks gate with output `wire` is a carry part of the adder circuit
    fn is_carry_circuit(&self, wire: WireLabel, n: u8) -> CircuitResult {
        if n == 0 {
            return CircuitResult::Valid;
        }

        let gate = self.gate_with_output(wire);

        match gate {
            Gate {
                op: GateType::Or,
                left,
                right,
                ..
            } => {
                let inputs = self.find_gates(|gate| {
                    (gate.output == left || gate.output == right)
                        && gate.left[0] != b'x'
                        && gate.left[0] != b'y'
                });

                if inputs.len() != 1 {
                    return CircuitResult::Invalid(WiringError {
                        gate: inputs.iter().find(|g| g.op != GateType::And).copied(),
                        expected_op: GateType::And,
                    });
                }
                let carry = inputs[0];

                let carry_next = self.find_gates(|gate| {
                    (gate.output == carry.left || gate.output == carry.right)
                        && gate.left[0] != b'x'
                        && gate.left[0] != b'y'
                });

                if carry_next.is_empty() {
                    if n == 1 {
                        // for bit 1 this part of carry circuit is not needed
                        return CircuitResult::Valid;
                    } else {
                        return CircuitResult::Invalid(WiringError {
                            gate: Some(carry),
                            expected_op: GateType::Or,
                        });
                    }
                }
                let carry_next = carry_next[0];

                self.is_carry_circuit(carry_next.output, n - 1)
            }
            _ => {
                return CircuitResult::Invalid(WiringError {
                    gate: Some(gate),
                    expected_op: GateType::Or,
                });
            }
        }
    }

    // checks if the circuit starting at a given is a valid adder circuit for bit `n`
    fn validate_adder_for_bit(&self, label: WireLabel, n: u8) -> CircuitResult {
        let gate = match self.try_find_gate(label) {
            Some(gate) => gate,
            None => return CircuitResult::NotFound,
        };

        if gate.op != GateType::Xor {
            return CircuitResult::Invalid(WiringError {
                gate: Some(gate),
                expected_op: GateType::Xor,
            });
        }

        let inputs = self.find_gates(|g| (g.output == gate.left || g.output == gate.right));

        let or_gate = match inputs.iter().find(|g| g.op == GateType::Or) {
            Some(gate) => gate,
            None => {
                return CircuitResult::Invalid(WiringError {
                    gate: inputs
                        .iter()
                        .find(|g| g.left[0] != b'x' && g.left[0] != b'y')
                        .copied(),
                    expected_op: GateType::Or,
                });
            }
        };

        match self.is_carry_circuit(or_gate.output, n - 1) {
            CircuitResult::Valid => {}
            err => return err,
        };

        // check that the other gate is a XOR gate (ideally also having x and y as inputs)
        match inputs
            .iter()
            .find(|g| g.output != or_gate.output && g.op == GateType::Xor)
        {
            Some(gate) => gate,
            None => {
                return CircuitResult::Invalid(WiringError {
                    gate: inputs.iter().find(|g| g.op != GateType::Or).copied(),
                    expected_op: GateType::Xor,
                });
            }
        };

        CircuitResult::Valid
    }

    fn max_bit_for_var(&self, prefix: &str) -> u8 {
        let cnt = (0..)
            .into_iter()
            .take_while(|i| self.values.contains_key(&label_from_u8(&prefix, *i)))
            .count();
        (cnt - 1) as u8
    }

    // runs the check if z is an adder circuit, returns the first invalid gate
    fn validate_adder(&self, start_at_bit: u8) -> (CircuitResult, u8) {
        let max_bit = self.max_bit_for_var("z");
        let mut n = start_at_bit;

        while let Some(gate) = self.try_find_gate(label_from_u8("z", n)) {
            if n >= max_bit {
                break;
            }

            match self.validate_adder_for_bit(gate.output, n) {
                CircuitResult::Valid => {}
                CircuitResult::NotFound => {
                    return (CircuitResult::Valid, n);
                }
                err => {
                    return (err, n);
                }
            }

            n += 1;
        }

        (CircuitResult::Valid, n)
    }

    // tries to find which 2 wires to swap
    fn find_fix(&self, error: &WiringError) -> Option<(WireLabel, WireLabel)> {
        match error {
            WiringError {
                gate: Some(gate),
                expected_op,
            } => {
                let inputs = [gate.left, gate.right];
                match self
                    .find_gates(|g| {
                        inputs.contains(&g.left)
                            && inputs.contains(&g.right)
                            && &g.op == expected_op
                    })
                    .as_slice()
                {
                    [swap_with] => {
                        return Some((gate.output, swap_with.output));
                    }
                    _ => {
                        return None;
                    }
                }
            }
            _ => None,
        }
    }

    fn find_fix_with_hint(
        &self,
        hint1: &WiringError,
        hint2: &WiringError,
    ) -> Option<(WireLabel, WireLabel)> {
        if let (Some(g1), Some(g2)) = (hint1.gate, hint2.gate) {
            if g1.op == hint2.expected_op && g2.op == hint1.expected_op {
                return Some((g1.output, g2.output));
            }
        }
        None
    }

    fn check_and_fix(&self) -> Option<(WireLabel, WireLabel)> {
        match self.validate_adder(2) {
            (CircuitResult::Invalid(error), n) => {
                let hint1 = &error;
                // try running further to get a hint
                if let (CircuitResult::Invalid(hint2), n2) = self.validate_adder(n + 1) {
                    if n2 == n + 1 {
                        return self.find_fix_with_hint(hint1, &hint2);
                    }
                }
                // try direct replacement
                if let Some(fix) = self.find_fix(&error) {
                    return Some(fix);
                }
            }
            _ => {}
        };
        None
    }

    // Task #2 - so I could just find the broken bits and then bruteforce it, but no, implemented
    // this monstrostiy that checks whether the circuit is a binary adder, reporting unexpected
    // gates (disclaimer: this is not a generalized solution, there are cicuits that do adding but
    // don't fit expected schema)
    fn find_crossed_wires(&mut self) -> u64 {
        self.reset_values();
        self.eval();

        let mut output_swaps = vec![];
        while let Some((from, to)) = self.check_and_fix() {
            info!(
                "found FIX swapping {:?} with {:?}",
                print_label(&from),
                print_label(&to)
            );
            self.rewire(&[(from, to)]);
            output_swaps.push((from, to));
        }

        let mut crossed_wires = output_swaps
            .into_iter()
            .map(|(a, b)| [print_label(&a), print_label(&b)])
            .flatten()
            .collect::<Vec<_>>();
        crossed_wires.sort();

        // formatted task solution
        println!("{}", crossed_wires.join(","));

        let mut hasher = DefaultHasher::new();
        crossed_wires.hash(&mut hasher);
        hasher.finish()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut wires = CrossedWires::from_input(input);

    let result = match part {
        day::Part::One => wires.find_z_value(),
        day::Part::Two => wires.find_crossed_wires(),
    } as i64;

    Ok(result)
}

day_tests!(
    "day_24-1.dat",
    51107420031718,
    2878072585763774253 /* cpm,ghp,gpr,krs,nks,z10,z21,z33 */
);
