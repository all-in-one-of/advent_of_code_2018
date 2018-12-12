// Matrix 2D backed by a vector
use crate::vec2::Vec2us;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Mat2<T: Clone> {
    data: Vec<T>,
    size: Vec2us,
}

impl<T: Clone> Mat2<T> {
    pub fn new(item: T, size: Vec2us) -> Self {
        Mat2 {
            data: vec![item; size.x * size.y],
            size,
        }
    }
}

impl<T: Clone> Index<usize> for Mat2<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &self.data[base..base + self.size.y]
    }
}

impl<T: Clone> IndexMut<usize> for Mat2<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &mut self.data[base..base + self.size.y]
    }
}
