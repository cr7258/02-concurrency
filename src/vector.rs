use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

// pretend this is a heavy operation, CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    // a.len => a.data.len() (Deref trait)
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

// 通过实现 Deref，我们可以在 Vector 上调用 Vec 的方法，比如 len、push、pop 等。
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    // 通过 Into<Vec<T>> trait bound，我们可以接受任何可以转换为 Vec<T> 的类型，比如数组。
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self {
            // data.into() 会将 data 转换为 Vec<T>
            data: data.into(),
        }
    }
}
