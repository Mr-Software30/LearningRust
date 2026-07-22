#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {

    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle {
        height: 40,
        width: 20,
    };

    let rect3 = Rectangle {
        height: 60,
        width: 40,
    };

    let rect4: Rectangle = Rectangle {
        height: 50,
        width: 25,
    };

    let square1 = Rectangle::square(20);

    println!(
        "The area of the rectangle is is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));

    //dbg!(&rect1); //unlike println!, dbg! does take ownership of the value.

    println!("\nRectangle1 is {:#?}", rect1);
    println!("Rectangle2 is {:#?}", rect2);
    println!("Rectangle3 is {:#?}", rect3);
    println!("Square1 is {:#?}", square1);
}

// fn calculate_area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }