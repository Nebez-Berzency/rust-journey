mod basic_loop;

fn main() {
    match basic_loop::run(4) {
        Ok(_) =>  println!("Success!"),
        Err(e) => println!("Error: {e}")
    }
}
