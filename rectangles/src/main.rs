#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is is {} square pixels.",
        calculate_area(&rect1)
    );

    println!("\nRectangle is {:#?}", rect1);
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}