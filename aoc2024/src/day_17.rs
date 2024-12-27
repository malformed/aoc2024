use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

use std::fmt::{self, Display, Formatter};

struct ComputerConfigReader {
    input: Input,
}

impl ComputerConfigReader {
    fn new(input: Input) -> Self {
        Self { input }
    }
}

impl ComputerConfigReader {
    fn read_register(&mut self) -> Option<i64> {
        self.input
            .read_line()
            .expect("button input")
            .split(':') // Register X: <value>
            .skip(1)
            .take(1)
            .map(|s| s.trim().parse::<i64>().expect("a number"))
            .next()
    }

    fn skip_line(&mut self) {
        let _ = self.input.read_line();
    }

    fn read_program(&mut self) -> Option<Vec<u8>> {
        self.input
            .read_line()
            .expect("program input")
            .split(':') // Program: <code>...
            .skip(1)
            .take(1)
            .map(|s| {
                s.trim()
                    .split(',')
                    .map(|s| s.parse::<u8>().expect("a number"))
                    .collect::<Vec<_>>()
            })
            .next()
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
}

impl std::ops::Index<Register> for Computer {
    type Output = i64;

    fn index(&self, reg: Register) -> &Self::Output {
        &self.registers[reg as usize]
    }
}

impl std::ops::IndexMut<Register> for Computer {
    fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
        &mut self.registers[reg as usize]
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Register::A => write!(f, "*A"),
            Register::B => write!(f, "*B"),
            Register::C => write!(f, "*C"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Arg {
    Register(Register),
    Literal(u8),
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Arg::Register(reg) => write!(f, "{}", reg),
            Arg::Literal(val) => write!(f, "{:#01x}", val),
        }
    }
}

impl Arg {
    fn literal(arg: u8) -> Self {
        Arg::Literal(arg)
    }

    fn combo(arg: u8) -> Self {
        match arg {
            0..=3 => Arg::Literal(arg),
            4 => Arg::Register(Register::A),
            5 => Arg::Register(Register::B),
            6 => Arg::Register(Register::C),
            7 => panic!("Reserved operand value: b111"),
            _ => panic!("Invalid operand: {:?}", arg),
        }
    }

    fn value(&self, computer: &Computer) -> i64 {
        match self {
            Arg::Register(reg) => computer[*reg],
            Arg::Literal(val) => *val as i64,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Adv(Arg),
    Bxl(Arg),
    Bst(Arg),
    Jnz(Arg),
    Bxc(Arg),
    Out(Arg),
    Bdv(Arg),
    Cdv(Arg),
}

impl Instr {
    fn new(opcode: u8, arg: u8) -> Self {
        match opcode {
            0 => Instr::Adv(Arg::combo(arg)),
            1 => Instr::Bxl(Arg::literal(arg)),
            2 => Instr::Bst(Arg::combo(arg)),
            3 => Instr::Jnz(Arg::literal(arg / 2)), // converts IP to asm
            4 => Instr::Bxc(Arg::literal(arg)),
            5 => Instr::Out(Arg::combo(arg)),
            6 => Instr::Bdv(Arg::combo(arg)),
            7 => Instr::Cdv(Arg::combo(arg)),

            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Instr::Adv(arg) => write!(f, "adv {}", arg),
            Instr::Bxl(arg) => write!(f, "bxl {}", arg),
            Instr::Bst(arg) => write!(f, "bst {}", arg),
            Instr::Jnz(arg) => write!(f, "jnz {}", arg),
            Instr::Bxc(arg) => write!(f, "bxc {}", arg),
            Instr::Out(arg) => write!(f, "out {}", arg),
            Instr::Bdv(arg) => write!(f, "bdv {}", arg),
            Instr::Cdv(arg) => write!(f, "cdv {}", arg),
        }
    }
}

type MachineCode = Vec<u8>;
type Asm = Vec<Instr>;
type Registers = [i64; 3];

struct Computer {
    code: MachineCode,
    asm: Asm,

    registers: Registers,
    ip: usize,

    debug: bool,

    output: Vec<u8>,

    jmp_flag: bool,
}

impl Computer {
    fn from_input(input: Input) -> Self {
        let mut reader = ComputerConfigReader::new(input);

        let reg_a = reader.read_register().expect("register A");
        let reg_b = reader.read_register().expect("register B");
        let reg_c = reader.read_register().expect("register C");

        reader.skip_line();

        let code = reader.read_program().expect("program");
        let asm = Self::disassemble(&code);

        Self {
            code,
            asm,
            registers: [reg_a, reg_b, reg_c],
            ip: 0,
            debug: false,
            output: vec![],
            jmp_flag: false,
        }
    }

    #[allow(dead_code)]
    fn new(asm: Asm, initial_registers: Registers, debug: bool) -> Self {
        Self {
            code: vec![],
            asm,
            registers: initial_registers,
            ip: 0,
            debug,
            output: vec![],
            jmp_flag: false,
        }
    }

    fn dbg(&self, msg: &str) {
        if self.debug {
            println!("{}", msg);
        }
    }

    fn disassemble(code: &MachineCode) -> Asm {
        code.chunks(2)
            .map(|instr| {
                if let [opcode, arg] = instr {
                    Instr::new(*opcode, *arg)
                } else {
                    panic!("Invalid instruction: {:?}", instr);
                }
            })
            .collect::<Vec<_>>()
    }

    fn print_state(&self) {
        println!("Registers: {:?}", self.registers);
        println!("Code: {:?}", self.code);
        println!("IP: {:?}", self.ip);
        println!("Output: {:?}", self.output);
    }

    fn exec_instr(&mut self, instr: Instr) {
        match instr {
            Instr::Adv(arg) => {
                let left = self[Register::A];
                let right = 2_i64.pow(arg.value(self) as u32);

                let res = left / right;

                self.dbg(&format!("adv ({arg}| {} / {} = {}", left, right, res));

                self[Register::A] = res;
            }

            Instr::Bxl(arg) => {
                let left = self[Register::B];
                let right = arg.value(self);

                let res = left ^ right;

                self.dbg(&format!("bxl ({arg})| {left} ^ {right} = {res}"));

                self[Register::B] = res;
            }

            Instr::Bst(arg) => {
                let x = arg.value(self);
                let res = arg.value(self) % 8;

                self.dbg(&format!("bst ({arg})| {} % 8 = {}", x, res));

                self[Register::B] = res;
            }

            Instr::Jnz(arg) => {
                let a = self[Register::A];

                if a != 0 {
                    let ip = arg.value(self) as usize;

                    self.dbg(&format!("jnz ({arg})| *A = {}, jump to {}", a, ip));

                    self.ip = ip / 2;
                    self.jmp_flag = true;
                } else {
                    self.dbg(&format!("jnz ({arg})| *A = {}, nop", a));
                }
            }

            Instr::Bxc(_) => {
                let b = self[Register::B];
                let c = self[Register::C];

                let res = b ^ c;

                self.dbg(&format!("bxc (_)| {} ^ {} = {}", b, c, res));

                self[Register::B] = res;
            }

            Instr::Out(arg) => {
                let val = arg.value(self);
                let res = val % 8;

                self.dbg(&format!(
                    "*out ({arg})| {} -> {} | {:?}",
                    val, res, self.output
                ));

                self.output.push(res as u8);
            }

            Instr::Bdv(arg) => {
                let left = self[Register::A];
                let right = 2_i64.pow(arg.value(self) as u32);

                let res = left / right;

                self.dbg(&format!("bdv ({arg})| {} / {} = {}", left, right, res));

                self[Register::B] = res;
            }

            Instr::Cdv(arg) => {
                let left = self[Register::A];
                let right = 2_i64.pow(arg.value(self) as u32);

                let res = left / right;

                self.dbg(&format!("cdv ({}) | {} / {} = {}", arg, left, right, res));

                self[Register::C] = res;
            }
        }
    }

    fn exec(&mut self) {
        while let Some(instr) = self.asm.get(self.ip) {
            self.exec_instr(*instr);

            if self.jmp_flag {
                self.jmp_flag = false;
            } else {
                self.ip += 1;
            }
        }
    }

    fn run_program(&mut self) -> i64 {
        self.exec();

        let output = self
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        println!("{}", output.join(","));

        output.join("").parse::<i64>().expect("a number")
    }

    fn reset(&mut self, a: i64) {
        self.registers = [a, 0, 0];
        self.ip = 0;
        self.output.clear();
    }

    fn test_a(&mut self, a: i64, expected_vec: &[u8]) -> bool {
        println!("testing A = {} ~ {:?} ", a, expected_vec);

        let mut a = a;

        for i in (0..expected_vec.len()).rev() {
            let expected = expected_vec[i];
            print!("\ttrying A = {} ~ {:?} ", a, expected);

            let amod8 = a % 8;
            let amod8xor5 = amod8 ^ 5;
            let res = (a / 2_i64.pow(amod8xor5 as u32) ^ amod8xor5 ^ 6) % 8;

            if res as u8 != expected {
                println!("=> false");
                return false;
            } else {
                println!("=> true");
            }
            a = a / 8;
        }

        true
    }

    fn find_a(&mut self) -> i64 {
        self.exec();

        let program = [2, 4, 1, 5, 7, 5, 4, 3, 1, 6, 0, 3, 5, 5, 3, 0];

        self.reset(0);

        let mut aa = 0;
        let mut result_a = 0;

        let mut expected: Vec<u8> = vec![];

        for out in program.iter().rev() {
            expected.push(*out);
            let mut a = aa;
            loop {
                if self.test_a(a, expected.as_slice()) {
                    aa = a * 8;
                    result_a = a;
                    break;
                }
                a += 1;
            }
        }

        println!("A to produce: {:?}: {}", expected, result_a);

        result_a
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let mut computer = Computer::from_input(input);

    computer.print_state();

    let result = match part {
        day::Part::One => computer.run_program(),
        day::Part::Two => computer.find_a(),
    };

    Ok(result)
}

#[cfg(test)]
mod test_instructions {
    use super::*;

    #[test]
    fn adv_instr() {
        let asm = vec![Instr::Adv(Arg::Literal(2))];

        let mut computer = Computer::new(asm, [11, 0, 0], true);
        computer.exec();

        assert_eq!(computer[Register::A], 2);
    }

    #[test]
    fn bxl_instr() {
        let asm = vec![Instr::Bxl(Arg::Literal(0b010))];

        let mut computer = Computer::new(asm, [0, 15, 0], true);
        computer.exec();

        assert_eq!(computer[Register::B], 13);
    }

    #[test]
    fn bst_instr_1() {
        let asm = vec![Instr::Bst(Arg::combo(2))];

        let mut computer = Computer::new(asm, [0, 0, 0], true);
        computer.exec();

        assert_eq!(computer[Register::B], 2);
    }

    #[test]
    fn bst_instr_2() {
        let asm = vec![Instr::Bst(Arg::combo(4))];

        let mut computer = Computer::new(asm, [39, 0, 0], true);
        computer.exec();

        assert_eq!(computer[Register::B], 7);
    }

    #[test]
    fn bxc_instr() {
        let asm = vec![Instr::Bxc(Arg::literal(0))];

        let mut computer = Computer::new(asm, [0, 15, 1], true);
        computer.exec();

        assert_eq!(computer[Register::B], 14);
    }

    #[test]
    fn out_instr() {
        let asm = vec![
            Instr::Out(Arg::combo(3)), // prints 3
            Instr::Out(Arg::combo(4)), // prints (*A % 8)
            Instr::Out(Arg::combo(5)), // prints (*B % 8)
            Instr::Out(Arg::combo(6)), // prints (*C % 8)
        ];

        let mut computer = Computer::new(asm, [100, 101, 102], true);
        computer.exec();

        assert_eq!(computer.output, vec![3, 4, 5, 6]);
    }

    #[test]
    fn bdv_instr() {
        let asm = vec![Instr::Bdv(Arg::combo(6))];

        let mut computer = Computer::new(asm, [33, 0, 3], true);
        computer.exec();

        assert_eq!(computer[Register::B], 4);
    }

    #[test]
    fn cdv_instr() {
        let asm = vec![Instr::Cdv(Arg::combo(5))];

        let mut computer = Computer::new(asm, [33, 2, 0], true);
        computer.exec();

        assert_eq!(computer[Register::C], 8);
    }

    #[test]
    fn cdv_instr_2() {
        let asm = vec![Instr::Cdv(Arg::combo(3))];

        let mut computer = Computer::new(asm, [33, 0, 0], true);
        computer.exec();

        assert_eq!(computer[Register::C], 4);
    }
}

day_tests!(
    "day_17-1.dat",
    735757430, /* 7,3,5,7,5,7,4,3,0 */
    105734774294938
);
