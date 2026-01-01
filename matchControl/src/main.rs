fn main() {
    let money = Coin::Penny;
    let value = value_in_cents(money);
    let money = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(money);
    println!("{value}");
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // can have arguments in enum types
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("WARNING: Penny going out of circulation!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {state:?}");
            25},
    }
}
