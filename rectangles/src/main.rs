fn main() {
    let rect1 = rectangle{length : 50, width : 30};
    println!("Area = {}", area(&rect1));
    println!("Rect = {rect1:?}");
}
#[derive(Debug)]
struct rectangle{
    length : u32,
    width : u32,
}

fn area(dimensions: &rectangle) -> u32 {
    dimensions.length * dimensions.width
}