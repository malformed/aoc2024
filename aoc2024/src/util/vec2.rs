#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn abs_vec(&self) -> Vec2 {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    pub fn wrapping_add_mut(&mut self, other: &Vec2, bounds: &Vec2) {
        *self = &(*self + other) % bounds;
    }

    // TODO: move this vec2 utils/tools or something
    pub fn neighbours(&self) -> [Vec2; 4] {
        [
            *self + (0, -1),
            *self + (0, 1),
            *self + (-1, 0),
            *self + (1, 0),
        ]
    }
}

impl std::ops::Mul<i64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: i64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Rem<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn rem(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x.rem_euclid(other.x),
            y: self.y.rem_euclid(other.y),
        }
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(t: (usize, usize)) -> Self {
        Self {
            x: t.0 as i64,
            y: t.1 as i64,
        }
    }
}

impl From<Vec2> for (usize, usize) {
    fn from(p: Vec2) -> Self {
        (p.x as usize, p.y as usize)
    }
}

impl std::str::FromStr for Vec2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i64> = s
            .split(&[',', ' ', '|'])
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()
            .map_err(|e| format!("invalid Vec2: {}", e))?;

        if let [x, y] = parts[..] {
            Ok(Self::new(x, y))
        } else {
            Err(format!("invalid Vec2: {:?}", parts))
        }
    }
}

impl<T> std::ops::Index<Vec2> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, pos: Vec2) -> &Self::Output {
        &self[pos.y as usize][pos.x as usize]
    }
}

impl<T> std::ops::IndexMut<Vec2> for Vec<Vec<T>> {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        &mut self[pos.y as usize][pos.x as usize]
    }
}

impl std::ops::Add<(i64, i64)> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: (i64, i64)) -> Vec2 {
        Vec2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        &self + &other
    }
}

impl std::ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        &self + other
    }
}

impl std::ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub type Pos = Vec2;
