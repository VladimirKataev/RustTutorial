fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let spaces = "   ";         // shadowing, 
    let spaces = spaces.len();  // we can redeclare a variable name with same name

    // let mut spaces = "   ";
    // spaces = spaces.len(); // CAN NOT change an existing variable's type though

}