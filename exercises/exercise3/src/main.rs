fn main(){
    let input:u32 = 4;
    let result:String = weekday_from_number(input);
    println!("Result: {}", result);
}

fn weekday_from_number(day: u32) -> String{
    // let mut dayInWords:&str = "Day"; 
    if day == 1{
        return String::from("Sunday");
    }else if day == 2{
        return String::from("Monday");
    }else if day == 3{
        return String::from("Tuesday");
    }else if day == 4{
        return String::from("Wednesday");
    }else if day == 5{
        return String::from("Thursday");
    }else if day == 6{
        return String::from("Friday");
    }else if day == 7{
        return String::from("Saturday");
    }else {
        return panic!("Invalid Day Number!");
    }
}