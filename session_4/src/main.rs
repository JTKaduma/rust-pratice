#[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    enum current_state{
        Kaduna,
        Kano,
        Abuja,
        Ogun,
        Lagos,
        Gombe,
        Kebbi,
    }

    #[derive(Debug)]
    struct User{
        name: String,
        age: i32,
        is_student: bool,
        residence: current_state,
    }
fn main(){

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));
    
    let Jason = User{
        name: String::from("Jason"),
        age: 18,
        is_student: true,
        residence: current_state::Ogun
    };

    let something = Some(Jason);


    println!("{:#?}", something);
}