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
    fn canHold(&self, other : &Rectangle) -> bool{
        ((self.width > other.width) && (self.length > other.length)) ||
        ((self.length > other.width) && (self.width > other.length)) // rotate 90deg

    }
    fn constructSquare(size : u32) -> Self{
        Self{
            width : size,
            length : size,
        }
    }
}



fn main() {
    let rect1 = Rectangle{length : 50, width : 30};
    let rect2 = Rectangle{length : 60, width : 40};
    let rect3 = Rectangle{length : 30, width : 30};
    let sq = Rectangle::constructSquare(20);
    println!("Rectangles are:\n\t{rect1:?},\n\t{rect2:?},\n\t{rect3:?},\n\t{sq:?}");
    println!("Can rect1 hold rect2? {}", rect1.canHold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.canHold(&rect1));
}

