use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor};

#[derive(Debug, Copy, Clone)]
pub struct Calculator<T> {
    x: T,
    y: T,
}

impl <T> Calculator<T> {
    pub fn new(x: T , y: T) -> Self {
        Self {
            x, y
        }
    }
}

pub trait AdditiveOperations<T> {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
}

impl<T> AdditiveOperations<T> for Calculator<T>
where
    T: Add<Output = T> + Sub<Output = T>,
{
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub trait MultiplicativeOperations<T> {
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

impl<T> MultiplicativeOperations<T> for Calculator<T>
where
    T: Mul<Output = T> + Div<Output = T>,
{
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

pub trait BitwiseOperations<T> {
    fn bitand(self, other: Self) -> Self;
    fn bitor(self, other: Self) -> Self;
    fn bitxor(self, other: Self) -> Self;
}

impl<T> BitwiseOperations<T> for Calculator<T>
where
    T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
{
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }

    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }

    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
        }
    }
}









