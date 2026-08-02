fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    // let _vec2 = vec![1, 3, 5, 7];

    vec1.push(1);
    vec1.push(3);
    vec1.push(5);
    vec1.push(7);

    println!("Vec: {:?}", vec1);

    // let third: &i32 = &vec1[2];
    // println!("The third element is {}", third);

    let third: Option<&i32> = vec1.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("nothing found my friend!"),
    }

}
