use crate::day;
use crate::error::Result;
use crate::input::Input;

use log::info;

use std::io;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Mul,
    Do,
    Dont,
    LeftParen,
    RightParen,
    Comma,
    Invalid,
}

struct Scanner<'a> {
    cursor: Peekable<std::str::Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            cursor: input.chars().peekable(),
        }
    }

    fn scan_next(&mut self) -> Option<Token> {
        while let Some(c) = self.cursor.peek() {
            match c {
                '(' => {
                    self.cursor.next();
                    return Some(Token::LeftParen);
                }
                ')' => {
                    self.cursor.next();
                    return Some(Token::RightParen);
                }
                ',' => {
                    self.cursor.next();
                    return Some(Token::Comma);
                }
                'm' => {
                    return self.read_mul();
                }
                'd' => {
                    return self.read_do_dont();
                }
                c if c.is_numeric() => {
                    return self.read_number();
                }
                _ => {
                    self.consume_invalid_sequence();
                    return Some(Token::Invalid);
                }
            }
        }

        return None;
    }

    fn valid_initial(c: char) -> bool {
        c == '(' || c == ')' || c == 'm' || c == 'd' || c.is_numeric()
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut content = String::new();
        while let Some(c) = self.cursor.peek() {
            if predicate(*c) {
                content.push(*c);
                self.cursor.next();
            } else {
                break;
            }
        }
        content
    }

    fn consume_invalid_sequence(&mut self) {
        self.consume_while(|c| !Self::valid_initial(c));
    }

    fn read_number(&mut self) -> Option<Token> {
        let num_str = self.consume_while(|c| c.is_numeric());

        match num_str.parse() {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => Some(Token::Invalid),
        }
    }

    fn read_mul(&mut self) -> Option<Token> {
        self.expect_symbol('m')
            .and_then(|_| self.expect_symbol('u'))
            .and_then(|_| self.expect_symbol('l'))
            .map(|_| Token::Mul)
            .or(Some(Token::Invalid))
    }

    fn read_do_dont(&mut self) -> Option<Token> {
        self.expect_symbol('d')
            .and_then(|_| self.expect_symbol('o'))?;

        match self.cursor.peek() {
            Some('(') => self.read_do(),
            Some('n') => self.read_dont(),
            _ => Some(Token::Invalid),
        }
    }

    fn read_do(&mut self) -> Option<Token> {
        self.expect_symbol('(')
            .and_then(|_| self.expect_symbol(')'))
            .map(|_| Token::Do)
    }

    fn read_dont(&mut self) -> Option<Token> {
        self.expect_symbol('n')
            .and_then(|_| self.expect_symbol('\''))
            .and_then(|_| self.expect_symbol('t'))
            .and_then(|_| self.expect_symbol('('))
            .and_then(|_| self.expect_symbol(')'))
            .map(|_| Token::Dont)
    }

    fn expect_symbol(&mut self, expected: char) -> Option<()> {
        if let Some(c) = self.cursor.peek() {
            if *c == expected {
                self.cursor.next();
                return Some(());
            }
        }
        None
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return self.scan_next();
    }
}

struct ComputerMemory {
    input: String,
}

impl ComputerMemory {
    fn new(mut input: Input) -> Self {
        let input = input.read_all();
        Self { input }
    }

    fn eval(&self, with_toggle: bool) -> i64 {
        let mut scanner = Scanner::new(&self.input).peekable();
        let mut acc = 0;
        let mut enabled = true;

        while let Some(token) = scanner.next() {
            match token {
                Token::Do => enabled = true,
                Token::Dont => enabled = false,
                Token::Mul => {
                    if let Some(x) = self.try_eval_mul(&mut scanner) {
                        acc += if enabled || !with_toggle { x } else { 0 };
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        acc
    }

    fn try_eval_mul(&self, scanner: &mut Peekable<Scanner>) -> Option<i64> {
        let left = Self::expect_token(scanner, Token::LeftParen)
            .and_then(|_| Self::expect_number(scanner))?;

        let right =
            Self::expect_token(scanner, Token::Comma).and_then(|_| Self::expect_number(scanner))?;

        Self::expect_token(scanner, Token::RightParen)?;

        Some(left * right)
    }

    fn expect_number(scanner: &mut Peekable<Scanner>) -> Option<i64> {
        if let Some(Token::Number(num)) = scanner.peek() {
            let num = Some(*num);
            scanner.next();
            return num;
        }
        None
    }

    fn expect_token(scanner: &mut Peekable<Scanner>, expected: Token) -> Option<()> {
        if let Some(token) = scanner.peek() {
            if *token == expected {
                scanner.next();
                return Some(());
            }
        }
        None
    }
}

pub fn run(input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let memory = ComputerMemory::new(input);

    let result = match part {
        day::Part::One => memory.eval(false),
        day::Part::Two => memory.eval(true),
    };

    writeln!(output, "{}", result)?;

    info!("Day done ✅");
    Ok(())
}
