fn main(){
    let input:i32 = 0;
    let result:String = check_number_sign(input);
    println!("The Number is: {}", result);
}

fn check_number_sign(x:i32) -> String{
    if x > 0{
        return String::from("positive");
    }else if x < 0{
        return String::from("negative");
    }else{
        return String::from("Zero");
    }
}