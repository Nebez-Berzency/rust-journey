
fn main() {
    let name :&str = "Nabaz";
    let age :u8 = 33;
    let height :f32 = 1.82;
    let is_learning_rust : bool = true;
    let favorite_letter : char = 'R';
    let total_projects :u8 = 5;
    let programming_languages :[&str; 3] = ["Rust" , "Java" , "Kotlin"];

    println!("========== User Profile ==========");
    println!();
    println!("Name : {}" , name);
    println!("Age : {}" , age);
    println!("Height : {}" , height);
    println!("Learning rust ? {}" , is_learning_rust);
    println!("Favorite letter : {}", favorite_letter);
    println!("Projects : {}", total_projects);
    println!("Languages : {:?}", programming_languages);
    println!();
    println!("==================================");
}
