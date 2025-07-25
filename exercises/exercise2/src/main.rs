fn main(){
    let n:u32 = 34;
    let y: u32 = fibonacci(n);
    println!("Result: {}", y);
}

fn fibonacci(n: u32) -> u32{
    if n == 0{
        return 0;
    }
    if n == 1{
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}