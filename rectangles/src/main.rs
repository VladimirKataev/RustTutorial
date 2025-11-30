#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32,
}




fn main() {
    let rect1 = Rectangle{length : 50, width : 30};
    println!("Area = {}", area(&rect1));
    println!("Rect = {rect1:?}");
    dbg!(&rect1);
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.length * dimensions.width
}