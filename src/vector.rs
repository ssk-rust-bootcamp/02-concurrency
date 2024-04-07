use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.data.len() != b.data.len() {
        return Err(anyhow!("Vectors must be of the same length"));
    }
    let mut sum = T::default();
    for i in 0..a.data.len() {
        sum += a.data[i] * b.data[i];
    }
    Ok(sum)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
