pub fn run(n :u32) {
    
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        print!("{}," , a);
        let next = a + b;
        a = b ;
        b = next;
    }
}
