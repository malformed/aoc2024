use crate::day;
use crate::error::Result;
use crate::input::Input;

use log::info;

use std::io;

#[derive(Clone, Copy)]
struct Dims {
    width: usize,
    height: usize,
}

type Pos = (usize, usize);

struct Cursor {
    x: usize,
    y: usize,
    dims: Dims,
}

impl Cursor {
    fn new(words: &XmasWords, start_at: Option<Pos>) -> Cursor {
        let (x, y) = start_at.unwrap_or((0, 0));
        Cursor {
            x,
            y,
            dims: words.dims,
        }
    }

    fn pos(&self) -> Pos {
        (self.x, self.y)
    }

    fn pos_unless(&self, pos: Pos) -> Option<Pos> {
        match self.pos() {
            p if p == pos => None,
            p => Some(p),
        }
    }

    fn move_to(&mut self, pos: Pos) -> &Self {
        self.x = pos.0;
        self.y = pos.1;
        self
    }
}

struct HorizontalIterator {
    cursor: Cursor,
}

impl HorizontalIterator {
    fn new(words: &XmasWords) -> HorizontalIterator {
        HorizontalIterator {
            cursor: Cursor::new(words, None),
        }
    }
}

impl Iterator for HorizontalIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let (x, y) = self.cursor.pos();
        let Dims { width, height } = self.cursor.dims;

        let x = (x + 1) % width;
        let y = if x == 0 { (y + 1) % height } else { y };

        self.cursor
            .move_to((x, y))
            .pos_unless((width - 1, height - 1))
    }
}

struct VerticalIterator {
    cursor: Cursor,
}

impl VerticalIterator {
    fn new(words: &XmasWords) -> VerticalIterator {
        VerticalIterator {
            cursor: Cursor::new(words, Some((0, 0))),
        }
    }
}

impl Iterator for VerticalIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let (x, y) = self.cursor.pos();
        let Dims { width, height } = self.cursor.dims;

        let y = (y + 1) % height;
        let x = if y == 0 { (x + 1) % width } else { x };

        self.cursor
            .move_to((x, y))
            .pos_unless((width - 1, height - 1))
    }
}

struct MajorDiagonalsIterator {
    cursor: Cursor,

    start: Pos, // where to restart diagonals
    end: Pos,
}

impl MajorDiagonalsIterator {
    fn new(words: &XmasWords) -> MajorDiagonalsIterator {
        let Dims { width, height } = words.dims;

        let start = (0, height - 1);
        MajorDiagonalsIterator {
            cursor: Cursor::new(words, Some(start)),
            start,
            end: (width - 1, 0),
        }
    }

    fn restart(&mut self) -> Pos {
        let (x0, y0) = &mut self.start;
        if *y0 > 0 {
            *y0 -= 1;
        } else {
            *x0 += 1;
        }
        self.start
    }
}

impl Iterator for MajorDiagonalsIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let (x, y) = self.cursor.pos();
        let Dims { width, height } = self.cursor.dims;

        let x = (x + 1) % width;
        let y = (y + 1) % height;

        let pos = if x == 0 || y == 0 {
            self.restart()
        } else {
            (x, y)
        };

        self.cursor.move_to(pos).pos_unless(self.end)
    }
}

struct MinorDiagonalsIterator {
    cursor: Cursor,

    start: Pos, // where to restart diagonals
    end: Pos,
}

impl MinorDiagonalsIterator {
    fn new(words: &XmasWords) -> MinorDiagonalsIterator {
        let Dims { width, height } = words.dims;
        MinorDiagonalsIterator {
            cursor: Cursor::new(words, None),
            start: (0, 0),
            end: (width - 1, height - 1),
        }
    }

    fn restart(&mut self) -> Pos {
        let (x0, y0) = &mut self.start;
        if *x0 < self.cursor.dims.width - 1 {
            *x0 += 1;
        } else {
            *y0 += 1;
        }
        self.start
    }
}

impl Iterator for MinorDiagonalsIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let (x, y) = self.cursor.pos();
        let Dims { width, height } = self.cursor.dims;

        let y = (y + 1) % height;
        let x = if x > 0 { x - 1 } else { width - 1 };

        let pos = if y == 0 || x == width - 1 {
            self.restart()
        } else {
            (x, y)
        };

        self.cursor.move_to(pos).pos_unless(self.end)
    }
}

struct XmasWords {
    data: Vec<Vec<u8>>,
    dims: Dims,
}

impl XmasWords {
    const BORDER_BYTE: u8 = b'_';

    fn new(mut input: Input) -> Self {
        let mut words = Vec::new();
        words.push(vec![]); // this will be replaced with a horizontal border once we know the width

        loop {
            let mut buffer = vec![Self::BORDER_BYTE];
            match input.read_line_as_bytes_into(&mut buffer) {
                Some(()) => {
                    let last_idx = buffer.len() - 1;
                    buffer[last_idx] = Self::BORDER_BYTE;
                    words.push(buffer);
                }
                None => {
                    break;
                }
            }
        }

        let width = words[words.len() - 1].len();
        let horiz_border = vec![Self::BORDER_BYTE; width];

        words[0] = horiz_border.clone();
        words.push(horiz_border);

        let height = words.len();

        Self {
            data: words,
            dims: Dims { width, height },
        }
    }

    fn at(&self, pos: Pos) -> u8 {
        self.data[pos.1][pos.0]
    }

    // Task #1
    fn find_xmas(&self) -> usize {
        let all_directions_iterator = HorizontalIterator::new(&self)
            .chain(VerticalIterator::new(&self))
            .chain(MajorDiagonalsIterator::new(&self))
            .chain(MinorDiagonalsIterator::new(&self));

        /*
         * The iterator returns the positions in all 4 directions.
         *
         *  .. it would be too easy to just collect all the data into a string and then count 'XMAS'
         *  and 'SAMX' like so:
         *
         *  ```
         * let all = all_directions_it.map(|pos| self.at(pos) as char).collect::<String>();
         * all.matches("XMAS").count() + all.matches("SAMX").count()
         * ```
         *
         * ... so here is a crude state automaton that does that in a single pass without allocating another buffer,
         * the accumulator is a tuple of state of XMAS progress, state of SMAX progress and current total count.
         *
         *        0    1    2    3    *
         * .0:       ['X', 'M', 'A', 'S']
         * .1:       ['S', 'A', 'M', 'X']
         * .2: running total
         */

        let (_, _, total) = all_directions_iterator.fold((0, 0, 0), |acc, pos| {
            match (self.at(pos), acc) {
                (b'X', (_, 3, t)) => (1, 0, t + 1), //      ?|'samX' -> +1
                (b'X', (_, _, t)) => (1, 0, t + 0), // 'Xmas'|?

                (b'M', (1, 2, t)) => (2, 3, t + 0), // 'xMas'|'saMx'
                (b'M', (1, _, t)) => (2, 0, t + 0), // 'xMas'|?
                (b'M', (_, 2, t)) => (0, 3, t + 0), //      ?|'saMx'

                (b'A', (2, 1, t)) => (3, 2, t + 0), // 'xmAs'|'sAmx'
                (b'A', (2, _, t)) => (3, 0, t + 0), // 'xmAs'|?
                (b'A', (_, 1, t)) => (0, 2, t + 0), //      ?|'sAmx'

                (b'S', (3, _, t)) => (0, 1, t + 1), // 'xmaS'|'Samx' -> +1
                (b'S', (_, _, t)) => (0, 1, t + 0), // 'xmaS'|?

                (_, (_, _, t)) => (0, 0, t),
            }
        });

        total
    }

    // Task #2 - after all the cursor/iterator abstraction it's refreshing to write a bunch of
    // bluntly straighforward C-style loops.
    //
    // TODO: this can start at 2,2 and end at width-2, height-2
    fn find_x_mas(&self) -> usize {
        let mut count = 0;
        for y in 1..self.dims.height - 1 {
            for x in 1..self.dims.width - 1 {
                count += if self.at((x, y)) == b'A' {
                    let a = (self.at((x - 1, y - 1)), self.at((x + 1, y + 1)));
                    let b = (self.at((x - 1, y + 1)), self.at((x + 1, y - 1)));

                    if (a == (b'M', b'S') || a == (b'S', b'M'))
                        && (b == (b'M', b'S') || b == (b'S', b'M'))
                    {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
            }
        }
        count
    }
}

pub fn run(input: Input, mut output: impl io::Write, part: day::Part) -> Result<()> {
    let station = XmasWords::new(input);

    let result = match part {
        day::Part::One => station.find_xmas(),
        day::Part::Two => station.find_x_mas(),
    };
    writeln!(output, "{}", result)?;

    info!("Day done ✅");
    Ok(())
}
