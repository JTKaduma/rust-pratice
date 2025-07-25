const GOAT:i32 = 44;
fn main() {
    let mut add = 44;
    println!("First: {add}");


    {
        let add = 66;
        println!("new add {}", add);
    }

    add = 16;
    println!("Global variable: {}", GOAT);


    println!("Second: {}", add);

    //Primitive Data Type

    let num1:u64 = 66;
    let num2:u8 = 77;

    let _add = num1 as u32 + num2 as u32;

    let _k = 99.0;

    let _character = '6';

    let _string:&str = "Jason Bahago";

    let _is_false = false;

    //Compound Data type
    //Tuple
    let mut data:(f64, &str, char, u8) = (4.5, "Jason",'Y' , 18);
    // let num = data.3;
    // let m = data.0;
    // let k = data.1;
    // let p = data.2;

    data.0 = 10.0;
    data.3 = 1;

    let (_, one, _, two) = data;

    println!("Index 1 {one}, index 2 {two}");
    // println!("Tuple type {:#?}", data);

    //Array
    let my_array = [22; 32];
    // let [k, l] = my_array;
    let k = my_array.is_empty();
    println!("array, {}", k);

    //String
    let mut words :String = String::from("Hello World");
    words.push('S');
    words.push_str("  How are you?");
    println!("Result: {}", words);

    //Vector
    let mut vector = Vec::new();
    vector.push(88);

    //COnditonal Statement
    let number = 32;

    if number == 32 {
        println!("Hello");
    }else {
        println!("not good!");
    }
}

// fn str_to_string(x: &str)->String{
//     x.to_string()
// }

// fn string_to_str(x: &String) -> &str{
//     x.as_str()
// }