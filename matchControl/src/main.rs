fn main() {
    let money = Coin::Penny;
    let value = value_in_cents(money);
    println!("{value}");
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("WARNING: Penny going out of circulation!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
