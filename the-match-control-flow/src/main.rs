#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny_cents = value_in_cents(Coin::Penny);
    let quarter_cents = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("{}", penny_cents);
    println!("{}", quarter_cents);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:#?}", six);
    println!("{:#?}", none);

    let n = Some(8);

    if let Some(i) = n {
        println!("{}", i);
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State of quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{}", count);
}
