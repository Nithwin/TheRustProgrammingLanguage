enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }


// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let Coin::Quarter(state) = coin else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

// fn main() {
//     let coin = Coin::Penny;
//         let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {max}");
//     }

//     println!("{}", value_in_cents(coin));
//     println!("Hello, world!");
// }
