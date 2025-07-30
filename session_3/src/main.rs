struct Shape {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Shape {
        width: 2,
        height: 4,
    };

    println!("Area: {}", area(&rectangle));
}

fn area(rectangle: &Shape) -> u32 {
    rectangle.width * rectangle.height
}
