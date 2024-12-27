use std::collections::{HashMap, HashSet};

use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

struct SecretGenerator {
    value: u64,
    modulo: u64,
}

impl SecretGenerator {
    fn new(value: u64) -> Self {
        Self {
            value,
            modulo: 16777216,
        }
    }

    fn next_bare(mut value: u64) -> u64 {
        let mut tmp = value * 64; // shift left by 6
        tmp = value ^ tmp; // XOR
        let new_secret = tmp % 16777216;
        value = new_secret;

        // println!("{} -> {}", value, new_secret);

        tmp = value / 32;
        tmp = value ^ tmp;
        let next_secret = tmp % 16777216;
        value = next_secret;

        // println!("{} -> {}", new_secret, next_secret);

        tmp = value * 2048;
        tmp = value ^ tmp;
        let final_secret = tmp % 16777216;
        value = final_secret;

        // println!("{} -> {}", next_secret, final_secret);

        value
    }

    fn next_advanced(value: u64) -> u64 {
        (value * 48271) % 2147483647
    }

    fn next(&mut self) -> u64 {
        let n1 = Self::next_bare(self.value);
        /*
        let n2 = Self::next_advanced(self.value);
        if n1 == n2 {
            self.value = n1;
        } else {
            panic!("next secret mismatch");
        }
        */
        self.value = n1;
        self.value
    }

    fn nth(&mut self, n: u64) -> u64 {
        for _ in 0..n {
            self.next();
        }
        self.value
    }
}

type Secrets = Vec<u64>;

// TODO: make this works with references .. or rather play with referencing internal data in a type
type SellPricesMap = HashMap<[i8; 4], i8>; // sell price diff windows of size 4 -> sell prices

struct MonkeyBroker {
    seed: u64,
    prices: Vec<i8>,
    diffs: Vec<i8>,
    sell_prices: SellPricesMap,
}

impl MonkeyBroker {
    fn new(seed: u64, n: u64) -> Self {
        let mut gen = SecretGenerator::new(seed);

        let mut secrets = vec![seed];
        secrets.extend((0..n).map(|_| gen.next()).collect::<Vec<_>>());

        let prices = secrets
            .iter()
            .map(|&secret| (secret % 10) as i8)
            .collect::<Vec<_>>();

        let diffs = prices
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>();

        let sell_prices = SellPricesMap::new();

        Self {
            seed,
            prices,
            diffs,
            sell_prices,
        }
        .analyze_prices()
    }

    fn nth_secret(&self, n: u64) -> u64 {
        SecretGenerator::new(self.seed).nth(n)
    }

    fn analyze_prices(mut self) -> Self {
        self.diffs.windows(4).enumerate().for_each(|(i, window)| {
            let key: [i8; 4] = window[0..=3]
                .try_into()
                .expect("windows are always of size 4");

            let price = self.prices[i + 4];

            self.sell_prices.entry(key).or_insert(price);
        });
        self
    }

    fn sell_price(&self, seq: &[i8; 4]) -> Option<i8> {
        self.sell_prices.get(seq).copied()
    }
}

struct MonkeyStockExchange {
    iterations: u64,
    brokers: Vec<MonkeyBroker>,
}

impl MonkeyStockExchange {
    fn new(input: Input, n: u64) -> Self {
        let brokers = input
            .lines()
            .map(|line| line.expect("valid input").parse::<u64>().expect("a number"))
            .map(|seed| MonkeyBroker::new(seed, n))
            .collect::<Vec<_>>();
        Self {
            iterations: n,
            brokers,
        }
    }

    // Task #1
    fn find_secrets(&self) -> u64 {
        self.brokers
            .iter()
            .map(|broker| broker.nth_secret(self.iterations))
            .sum()
    }

    fn seq_sell_price(&self, seq: &[i8; 4]) -> i64 {
        self.brokers
            .iter()
            .map(|broker| broker.sell_price(seq).unwrap_or(0) as i64)
            .sum()
    }

    fn find_sell_sequence(&self) -> u64 {
        let mut best_sell_sequence = [0, 0, 0, 0];
        let mut best_sell_price = 0;
        let mut checked_seq_cache = HashSet::new();

        for (_i, broker) in self.brokers.iter().enumerate() {
            // println!("broker {}/{}", i, self.brokers.len());
            for seq in broker.sell_prices.keys() {
                if !checked_seq_cache.insert(*seq) {
                    continue;
                }

                let sell_price = self.seq_sell_price(seq);
                if sell_price > best_sell_price {
                    best_sell_price = sell_price;
                    best_sell_sequence = *seq;
                }
            }
        }

        println!("best sell sequence: {:?}", best_sell_sequence);
        let sell_price = self.seq_sell_price(&best_sell_sequence);

        sell_price as u64
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let iterations = 2000;
    let mse = MonkeyStockExchange::new(input, iterations);

    let result = match part {
        day::Part::One => mse.find_secrets(),
        day::Part::Two => mse.find_sell_sequence(),
    } as i64;

    Ok(result)
}

day_tests!("day_22-1.dat", 15608699004, 1791);
