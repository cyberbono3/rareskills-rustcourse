
pub mod calculator;
use calculator::*;

fn main() {
    let calc1 = Calculator::new(5, 10 );
    let calc2 = Calculator::new(3, 2);

    // Additive operations
    let sum = calc1.add(calc2);
    println!("Sum: {:?}", sum);

    let diff = calc1.sub(calc2);
    println!("Difference: {:?}", diff);

    // Multiplicative operations
    let calc1 = Calculator::new(5, 10 );
    let calc2 = Calculator::new(3, 2 );

    let product = calc1.mul(calc2);
    println!("Product: {:?}", product);

    let quotient = calc1.div(calc2);
    println!("Quotient: {:?}", quotient);

    // Bitwise operations
    let calc1 = Calculator::new(5, 10 );
    let calc2 = Calculator::new(3, 2 );

    let and_result = calc1.bitand(calc2);
    println!("And: {:?}", and_result);

    let or_result = calc1.bitor(calc2);
    println!("Or: {:?}", or_result);

    let xor_result = calc1.bitxor(calc2);
    println!("Xor: {:?}", xor_result);
}