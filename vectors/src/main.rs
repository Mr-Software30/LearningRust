fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    // let _vec2 = vec![1, 3, 5, 7];

    vec1.push(1);
    vec1.push(3);
    vec1.push(5);
    vec1.push(7);

    println!("Vec: {:?}", vec1);

    vec1.remove(2);
    println!("Vec: {:?}", vec1);
}
