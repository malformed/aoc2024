/*
#[derive(Debug, Clone, Copy)]
enum Keypad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Keypad {
    fn as_int(&self) -> usize {
        match self {
            Keypad::Zero => 0,
            Keypad::One => 1,
            Keypad::Two => 2,
            Keypad::Three => 3,
            Keypad::Four => 4,
            Keypad::Five => 5,
            Keypad::Six => 6,
            Keypad::Seven => 7,
            Keypad::Eight => 8,
            Keypad::Nine => 9,
            Keypad::A => 10,
        }
    }
}

impl std::ops::Index<(Keypad, Keypad)> for KeypadTable {
    type Output = Vec<Dir>;

    fn index(&self, index: (Keypad, Keypad)) -> &Self::Output {
        let (from, to) = index;
        &self.table[from.as_int()][to.as_int()]
    }
}

impl std::ops::IndexMut<(Keypad, Keypad)> for KeypadTable {
    fn index_mut(&mut self, index: (Keypad, Keypad)) -> &mut Self::Output {
        let (from, to) = index;
        &mut self.table[from.as_int()][to.as_int()]
    }
}
*/
