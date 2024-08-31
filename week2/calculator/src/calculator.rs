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

pub trait AdditiveOperations {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
}

impl<T> AdditiveOperations for Calculator<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
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

pub trait MultiplicativeOperations {
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

impl<T> MultiplicativeOperations for Calculator<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy,
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

pub trait BitwiseOperations {
    fn bitand(self, other: Self) -> Self;
    fn bitor(self, other: Self) -> Self;
    fn bitxor(self, other: Self) -> Self;
}

impl<T> BitwiseOperations for Calculator<T>
where
    T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy,
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

// Advanced trait
pub trait Advanced: AdditiveOperations + MultiplicativeOperations + BitwiseOperations{}

impl<T> Advanced for Calculator<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Copy,
{
}

pub fn print_output<T>(input: T)
where
    T: Advanced + Copy + std::fmt::Debug, // Ensure T implements Advanced, is Copy, and Debug
{
    let sum = input.add(input);
    let diff = input.sub(input);
    let product = input.mul(input);
    let quotient = input.div(input);
    let and_result = input.bitand(input);
    let or_result = input.bitor(input);
    let xor_result = input.bitxor(input);

    println!("Input: {:?}", input);
    println!("Sum: {:?}", sum);
    println!("Difference: {:?}", diff);
    println!("Product: {:?}", product);
    println!("Quotient: {:?}", quotient);
    println!("Bitwise AND: {:?}", and_result);
    println!("Bitwise OR: {:?}", or_result);
    println!("Bitwise XOR: {:?}", xor_result);
}













