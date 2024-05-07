use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Mul};

// pretend this is a heavy operation, CPU intensive
pub fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}
