fn main() {

    let four = IpAddrKind::V4;

    let home = IpAddrKind::V4(127,0,0,1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let quarter = Coin :: Quarter(UsState :: Alaska);
    
    if let Coin::Quarter(state) = quarter {
        println!("State quarter from {:?}!", state);
    }


    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Dakota,
    Ohio,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> i32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}


