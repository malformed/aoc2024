#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> [Pos; 4] {
        [
            *self + (0, -1),
            *self + (0, 1),
            *self + (-1, 0),
            *self + (1, 0),
        ]
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(t: (usize, usize)) -> Self {
        Self {
            x: t.0 as i64,
            y: t.1 as i64,
        }
    }
}

impl From<Pos> for (usize, usize) {
    fn from(p: Pos) -> Self {
        (p.x as usize, p.y as usize)
    }
}

impl<T> std::ops::Index<Pos> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self[pos.y as usize][pos.x as usize]
    }
}

impl<T> std::ops::IndexMut<Pos> for Vec<Vec<T>> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self[pos.y as usize][pos.x as usize]
    }
}

impl std::ops::Add<(i64, i64)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i64, i64)) -> Pos {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl std::ops::Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<&Pos> for Pos {
    type Output = Pos;

    fn sub(self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub type Vec2 = Pos;
