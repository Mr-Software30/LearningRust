fn main() {
    let s = String::from("hello");  // s comes into scope

    //s.push_str(" my friend");     // s is not mut  
    println!("{s}");

    takes_ownership(s);             // s's value moves into the function, and so is no longer valid here

    //println!("{s}");                // error

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    println!("{}" , x + 2);



    let mut s = String::from("hello");

        {
            let r1 = &mut s;

            r1.push_str(" world");
            println!{"r1 is: {r1}"};
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        {    
            let r2 = &mut s;

            r2.push_str(", and goodbye");
            println!{"r2 is: {r2}"};
        }

    println!{"final string is: {s}"};


} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.