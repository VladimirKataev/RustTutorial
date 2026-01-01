enum IpAddrKind{
    V4,
    V6
}

fn route(ip_kind: IpAddrKind) {}



enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move {x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        println!("Enum method called");
    }
}

fn main() {
    println!("Hello, Enums!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr::V4(127,0,0,1);
    
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Quit;
    m.call();

    let some_number = Some(5); // this is Option<i32>::some. Is option
    let some_char = Some('e'); // this is Option<char>::some. Is option

    let absent_number: Option<i32> = None; // Is option

    // let result = 5 + some_number; // FAILS
    // no way to add i32 and Option. 

}
