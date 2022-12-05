fn main() {
    ip_addr();
    option_enum();
    match_expressions();
    match_if_expressions();
}

fn ip_addr(){
    enum IpAddrKind {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn some_function(){
            print!("Hello");
        }
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let locahost = IpAddr {
        kind: IpAddrKind::V4(127,0,0,1),
        address: String::from("127.0.0.1")
    };
}

fn option_enum() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

fn adding_optional_value() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
}

fn match_expressions(){
    let coin = Coin::Quarter(UsState::Alaska);
    match_value_in_coins(coin);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_value_in_coins(coin: Coin){
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
        },
        Coin::Nickel => println!("Lucky nickel!"),
        Coin::Dime => println!("Lucky dime!"),
        Coin::Quarter(state) => {
            print!("Lucky quarter from {:?}!", state);
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

fn match_if_expressions(){
    use_plus_one();
    if_lets_syntax();
}

fn if_lets_syntax(){
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn use_plus_one(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // _ is like default in switch
        _ => None,
    }
}
