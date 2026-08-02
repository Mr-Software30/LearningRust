
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 3, 5, 7];

    vec1.push(1);
    vec1.push(3);
    vec1.push(5);
    vec1.push(7);

    // println!("Vec: {:?}", vec1);

    for i in &vec1 {
        println!("{i}");
    }

    println!("\n--------------------\n");
    // mutable reference
    for i in &mut vec2 {
        *i += 1; // * dereference operator to get to the value in i
        println!("{i}");
    }



    // let third: &i32 = &vec1[2];
    // println!("The third element is {}", third);

    let third: Option<&i32> = vec1.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("nothing found my friend!"),
    }

    println!("\n--------------------\n");

    let row1 = vec![
        SpreadSheetCell::Int(2),
        SpreadSheetCell::Text(String::from("warzer")),
        SpreadSheetCell::Float(70.25),
    ];

    // super exhaustive match needed here
    for i in &row1 {
        match i {
            SpreadSheetCell::Int(i) => println!("Int: {i}"),
            SpreadSheetCell::Text(s) => println!("Text: {s}"),
            SpreadSheetCell::Float(f) => println!("Float: {f}"),
        }
    }

}
/*
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");

    might look like it should work: Why should a reference to the first element 
    care about changes at the end of the vector? 
    This error is due to the way vectors work: 
    Because vectors put the values next to each other in memory, 
    adding a new element onto the end of the vector might require allocating new memory and 
    copying the old elements to the new space, 
    if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 
    In that case, the reference to the first element would be pointing to deallocated memory. 
    The borrowing rules prevent programs from ending up in that situation.
*/