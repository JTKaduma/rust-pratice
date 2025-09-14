#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum current_state {
    Kaduna,
    Kano,
    Abuja,
    Ogun,
    Lagos,
    Gombe,
    Kebbi,
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    is_student: bool,
    residence: current_state,
    monthly_data: i32,
}

#[derive(Debug)]
enum Network {
    Airtel,
    MTN,
    GLO,
    Etisalat,
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));

    let User1 = User {
        name: String::from("Jason"),
        age: 18,
        is_student: true,
        residence: current_state::Ogun,
        monthly_data: network_select(Network::GLO),
    };

    let User2 = User {
        name: String::from("Jemimah"),
        age: 14,
        is_student: true,
        residence: current_state::Kaduna,
        monthly_data: network_select(Network::MTN),
    };

    let User3 = User {
        name: String::from("Phinehas"),
        age: 45,
        is_student: false,
        residence: current_state::Kano,
        monthly_data: network_select(Network::Airtel),
    };

    let users = [User1, User2, User3];

    let something = Some(users);

    println!("{:#?}", something);

    println!("/////////");

    let num = 32;
    let some_num = Some(32);
    // let num2 =
    // let some_sum = num + some_num;

    // let data_base_price = network_select(network);

    let name = "Jason";
    let school = String::from("Babcock");
    let initial = 'j';
}

fn network_select(network: Network) -> i32 {
    match network {
        Network::Airtel => 2500,
        Network::Etisalat => 900,
        Network::GLO => 2000,
        Network::MTN => 3500,
    }
}
