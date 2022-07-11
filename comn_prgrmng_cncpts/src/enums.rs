enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enums_declarations() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("198.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("198.0.0.1"),
    };

    // Option Enum
    let some_number1 = Some(6);
    let some_number2 = Some(6);
    let value = match some_number1 {
        Some(value) => match some_number2 {
            Some(value2) => value + value2,
            None => {
                println!("Empty");
                1
            }
        },
        None => {
            println!("Empty");
            10
        }
    };
    println!("{}", value);
    let some_string = Some("a string");
    let absent_number: Option<u32> = None;

    // handling exhaustive match
    let dice_roll = 1;
    match dice_roll {
        3 => 3 + 1,
    }
}
