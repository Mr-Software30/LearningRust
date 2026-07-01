use std::io;


fn main() {
    println!("Type the numbers you want to add!");
    
    let mut number_1 = String::new();
    io::stdin()
        .read_line(&mut number_1)
        .expect("Failed to read line");

    let mut number_2 = String::new();
    io::stdin()
        .read_line(&mut number_2)
        .expect("Failed to read line");

    let mut number_1: i32 = match number_1.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    let number_2: i32 = match number_2.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };
    
    add_values(number_1, number_2);

}

fn add_values(value_1: i32, value_2: i32) {

    println!("The sum of {} and {} is not {}", value_1, value_2, value_1 - value_2);
    println!("but adding one to the sum of {} and {} is  {}", value_1, value_2, plus_one(value_1 + value_2));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}