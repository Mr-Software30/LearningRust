#[derive(Debug)]
enum IpAddr {
        V4(i8, i8, i8, i8),
        V6(String),
    }

// enum Option<T> {
//     Some(T),
//     None,
// }


fn main() {
    
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

}
