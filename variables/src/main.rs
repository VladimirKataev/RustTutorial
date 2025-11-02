use std::io;

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


    // types:
    // u32, i64, char, bool, isize & usize, etc for primitives (no doubles?) 
    // overflows are checked for on debug builds, and there are workarounds for releases
    // (i64, bool, char) for tuples
    

    // arrays can be done using the following:
    let mut a : [i32; 5] = [1,2,3,4,5];
    a[3] = -1;
    let a = [3 ; 5]; // for initial creation

    // out of bounds are checked for at compile: 
    // println!("{}", a[5]); // causes a crash

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}