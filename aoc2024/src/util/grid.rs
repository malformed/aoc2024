use super::Vec2;

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    dims: Vec2,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };

        debug_assert!(
            data.iter().all(|row| row.len() == width),
            "grid rows have different lengths"
        );

        Self {
            data,
            dims: (width, height).into(),
        }
    }

    pub fn dims(&self) -> Vec2 {
        self.dims
    }

    pub fn width(&self) -> usize {
        self.dims.x as usize
    }

    pub fn height(&self) -> usize {
        self.dims.y as usize
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter::new(self)
    }
}

impl<T> std::ops::Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Vec2) -> &Self::Output {
        &self.data[pos.y as usize][pos.x as usize]
    }
}

impl<T> std::ops::IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        &mut self.data[pos.y as usize][pos.x as usize]
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    pos: Vec2,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            pos: Vec2::default(),
        }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Vec2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y >= self.grid.dims.y {
            return None;
        }

        let pos = self.pos;
        self.pos.x += 1;

        if self.pos.x >= self.grid.dims.x {
            self.pos.x = 0;
            self.pos.y += 1;
        }

        Some((pos, &self.grid[pos]))
    }
}

// mut iterator
pub struct GridIterMut<'a, T> {
    grid: &'a mut Grid<T>,
    pos: Vec2,
}

impl<'a, T> GridIterMut<'a, T> {
    pub fn new(grid: &'a mut Grid<T>) -> Self {
        Self {
            grid,
            pos: Vec2::default(),
        }
    }
}
