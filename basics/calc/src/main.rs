// This Calculator uses i32 for integer operations.
// Use f32 or f64 instead if decimal calculations are required.


// Implicit return
fn add(a:i32 , b :i32) -> i32 {
    a + b 
}

// Implicit return
fn subtract(a:i32 , b:i32) -> i32 {
    a - b 
}

// Explicit return
fn multiply(a:i32 , b:i32) -> i32 {
    return a * b;
}

//Explicit return
fn divide(a:i32 , b:i32) -> i32 {
    return a / b;
}

fn main() {
    
    let a = 20;
    let b = 10;
    println!("========== Calculator ==========");
    println!();
    println!("{a} + {b} = {}",add(a, b));
    println!("{a} - {b} = {}",subtract(a, b));
    println!("{a} x {b} = {}",multiply(a, b));
    println!("{a} / {b} = {}",divide(a, b));
    println!();
    println!("===============================");
}
