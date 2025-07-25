fn main(){
    let mut number:i32 = 0;
    let mut add:i32 = 0;

    loop{
        if number == 3{
            number = number + 1;
            continue;
        }else if number == 10{
            break;
        }

        add = add + 1;
        number = number + 1;
    }

    println!{"Number: {} Add: {}", number, add};

    //To break from multiple loops through the use of a label

    let mut count: i32 = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("THe value is: {}", a[index]);

        index += 1;
    }

    //for loop
    for element in a{
        println!("The value is {element}")
    }

    //To loop through numbers
    // for c in 0..=10{
    //     println!("The element {c}")
    // }
    //To loop in reverse
    for c in (0..=10).rev(){
        println!("The element {c}")
    }

    //Ownership
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    //println!("{s1}, world") will return an error cause ownership has been changed unless .clone() is used
    println!("{s1}, world")
}
