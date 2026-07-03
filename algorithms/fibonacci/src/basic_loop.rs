pub fn run(n :u64) -> Result<(),String> {
    
    if n == 0 || n > 92 {
        return Err("Input must be between 1 and 92".to_string());
    }
    let mut a :u64 = 0;
    let mut b :u64 = 1;

    for i in 0..n {
    
        print!("{}" , a);
        
        if i < n-1 {
            print!(",")
        }
        
        let next = a + b;
        a = b ;
        b = next;
    }

    println!();

    Ok(())

}
