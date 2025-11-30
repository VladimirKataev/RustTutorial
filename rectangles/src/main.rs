#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32,
}
impl Rectangle{
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn width(&self) ->bool{
        self.width > 0
    }
}



fn main() {
    let rect1 = Rectangle{length : 50, width : 30};
    println!("Area = {}", rect1.area());
    println!("Rect = {rect1:?}");
    println!("is width above 0? {}", rect1.width());
    dbg!(&rect1);
}

