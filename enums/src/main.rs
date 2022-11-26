fn main() {
   let four = IpAddrKind::V4;
   //can have an empty varaible
   let absent_number: Option<i32> = None;
   let config_max = Some(3u8);
   if let Some(max) = config_max {
    println!("The maxim is configured to be {}" ,max);
   }
}

enum IpAddrKind {
    V4,//can do (String)
    V6,
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, Y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabamba,
    Alaska,
    Arizona,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}