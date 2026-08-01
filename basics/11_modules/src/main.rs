// mod keyword tells Rust we have a math module
mod math;

fn main() {
    
    let a = 20;
    let b = 10;

    let result = math::add(a ,b);

    println!("\n========================\n");
    println!("The result is: {}", result);
    println!("\n========================\n");
}
