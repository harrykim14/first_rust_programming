enum IpAddressType {
    V4, // 열거자의 열것값
    V6,
}

fn main() {
    // let four = IpAddressType::V4;
    // let six = IpAddressType::V6;
    let home = IpAddr {
        kind: IpAddressType::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddressType::V6,
        address: String::from("::1"),
    };
    let get_coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(get_coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    placeholder();
    iflet();
}

struct IpAddr {
    kind: IpAddressType,
    address: String,
}

fn route(ip_type: IpAddressType) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // match는 반드시 모든 경우를 처리해야 함
        None => None,
    }
}

// 모든 경우를 다 처리하고 싶지 않을 때엔 _ 자리지정자로 대체하면 된다
fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ 패턴은 모든 값에 일치함
    }
}

// if let 문법은 여러 경우 중 한 가지만 처리하고 나머지는 고려하고 싶지 않을 때 사용한다
// if let 문법은 else 표현식으로도 사용할 수 있다
fn iflet() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    } else {
        println!("not three...");
    }
}
