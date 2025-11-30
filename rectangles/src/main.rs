#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32,
}
impl Rectangle{
    fn area(&self) -> u32 {
        self.length * self.width
    }
}



fn main() {
    let rect1 = Rectangle{length : 50, width : 30};
    println!("Area = {}", rect1.area());
    println!("Rect = {rect1:?}");
    dbg!(&rect1);
}

