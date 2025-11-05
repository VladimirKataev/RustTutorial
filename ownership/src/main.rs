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

}
