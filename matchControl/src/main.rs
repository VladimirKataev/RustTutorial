fn main() {
    let money = Coin::Penny;
    let value = value_in_cents(money);
    let money = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(money);
    println!("{value}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let dice_roll = 9;

    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other),
    // }

    // Alternatively, if we don't care about other's value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // for a do-nothing operation
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        match (describe_state_quarter(Coin::Quarter(state))) {
            Some(String) => println!("{}", String),
            None => println!("Not Seen this"),
        }
    } else {
        count += 1;
    }


}



fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // can have arguments in enum types
}

// This version WILL COMPILE
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// This version WILL NOT COMPILE
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x { // This match MUST BE EXHAUSTIVE
//         Some(i) => Some(i + 1),
//     }
// }


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
