fn main() {
    println!("Hello, world!");
    let mut a : u32 = 67534654;
    // i8, u8, -> i128, u128. Arguably isize & usize
    // integer overflows are caught and panicked in debug mode
    // "normal" overflow happens in release

    let mut b : f32 = 98.4;
    //floats are stored with mantissa and exponent (and sign)
    //only f64 (default) and f32 though

    let c : bool = true;
    //bools are bools

    let d = '☯'; //chars are UTF-8
    // if char is 4 bytes, wtf???
    // how to individual memory? There is no cheese and the west has fallen.

    let tup : (i32, f32, char) = (67, 6.9, '☬');
    // typed tuples, like that.

    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr = [0, 0, 0, 0, 71];
    let arr = [3;5];

    println!("{}", arr[4]);


    // arrays need constant, or compile-time-known sizes. code below does not compile

    // let mut l = String::new();
    //
    // println!("Enter your arbitrary array length:");
    // std::io::stdin()                 
    //     .read_line(&mut l)      
    //     .expect("Failed to read line"); 
    // let l : u32 = 
    //     match l.trim().parse(){
    //         Ok(l) => l,
    //         Err(_) => 0, 
    //     };
    // let arr : [32; l];

    // run-time errors happen


    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; // THIS CAN PANIC

    println!("The value of the element at index {index} is: {element}");

}
