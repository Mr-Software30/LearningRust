fn main() {
    //--------- variables and const -------------

    // let x = 5; 
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let difference = 5 / 3;
    println!("The value of difference is: {difference}");

    //--------- shadowing -------------
    let z = 8;
    let z = z + 2;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z in the main fn scope is: {z}");
}
