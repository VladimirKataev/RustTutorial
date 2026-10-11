fn main() {
    println!("main()");    
    another_function(5);
    print_labeled_measurement(32, 'f');

    let y = { // this is unusual, but embedding raw-dog blocks is interesting
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("five() returns {}", five());
    println!("five()+1 returns {}", plus_one(five()));
}

fn another_function(x : i32) {
    println!("Another function with arg {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1
}