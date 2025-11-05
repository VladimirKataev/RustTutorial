fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // Nothing points to original, is dropped
    println!("{s}, world!");



    let s1 = String::from("hello");
    let s2 = s1.clone(); // "deep copy" implementation, using .clone(s)

    println!("s1 = {s1}, s2 = {s2}");


    // rust does a pure 'copy' trait for some types
    // -    namely the ones with a fixed compile time size
    // -    or the ones without a 'drop' trait


    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.