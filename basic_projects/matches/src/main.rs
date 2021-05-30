
#[derive(Debug)]
enum UsState {
    Alabama, Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("coin:{}", value_in_cents(coin));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => {
            println!("lucky Nickle!");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
