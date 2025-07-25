fn main(){
    let x = 32;
    println!("Before: {}", x);
    let y = numerical_type_conversion(x); 
    println!("After: {}", y);
}

fn numerical_type_conversion(x: i32) -> u32{
    return x as u32;
}