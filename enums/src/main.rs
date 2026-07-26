// use crate::UsState::Alaska;

#[derive(Debug)]
enum IpAddr {
        V4(i8, i8, i8, i8),
        V6(String),
    }

// enum Option<T> {
//     Some(T),
//     None,
// }
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}\n\n", loopback);

    value_in_cents(UsCoin::Quarter(UsState::Alaska));
    value_in_cents(UsCoin::Penny);
    value_in_cents(UsCoin::Dime);
    let coin1 = UsCoin::Penny;

    println!("value of a random coin is: {}", value_in_cents(UsCoin::Dime));
    println!("value of coin1: {} \n\n", value_in_cents(coin1)); // the matching patter will be displayed first, then the "value of coin1"

    let six = plus_one(Some(5));
    println!("six: {six:?}");

    let none = plus_one(None);
    println!("none: {none:?}");
}

fn value_in_cents(coin: UsCoin) -> u8 {
    match coin {
        UsCoin::Penny => {
            println!("Lucky penny huh?");
            1
        }
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // where are matching Option<i32> which either could be Some(i32) or None
        Some(i) => Some(i + 1),
        _ => None, // anything except Some will return None
    }
}
