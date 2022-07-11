use std::io;
// mod enumerations;
mod collections;
mod enumerations;
use crate::collections::vectors;
use crate::enumerations::enums_declarations;
// mod ownership;

// functions
fn _meal_calculator(items: [u32; 2]) {
    println!("{:?}", items);
    let y = {
        let z = 10;
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
        z + 1
    };
    println!("{y}");
}

// function with return value
fn _fun_with_return() -> u32 {
    let x = 10;
    x + 1
}

fn main() {
    // println!("Hello, world!");
    // variable_declarations();
    // meal_calculator([1, 3]);
    // println!("{}", fun_with_return());
    // control_blocks();
    // loops_in_rust();
    // ownership();
    // compound_datatypes();
    // ownership::ownership_main_1();
    // ownership::reference_main();
    enums_declarations();
    vectors();
}

fn _variable_declarations() {
    // Immutable Variable Declarations. By default variables will be immutable
    let x: u32 = 5;
    println!("The value of x is {}", x);
    // x = 6; it is not possible to re-assign the x value as it immutable;
    println!("The value of x is {}", x);

    // Mutable Variables Declarations
    let mut y: u32 = 6;
    println!("The value of y is {}", y);
    y = 12;
    println!("The value of y is {}", y);

    // Constant Declarations
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value f constant variable: {THREE_HOURS_IN_SECONDS}");

    // shadowing the variable
    let z = 10;
    let z = z + 1;
    {
        let z = z + 1;
        println!("z value is {}", z);
    };
    println!("z value is {}", z);

    // datatypes
    // u8, u16, u32, u64, u128
    let u: isize = 100_000;
    let v: usize = 0xff * 0o77 * 0b1111_0000;
    println!("U is {}, V is {}", u, v);

    // compound data types
    // 1. tuples
    let tup: (i32, char, bool) = (500, 'a', true);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    // 2. Arrays
    let _a = [1, 2, 3, 4, 5]; // compiler allow unused variables if it is prefixed with _
    let a = [3; 5]; // will result in [3,3,3,3,3]
    let b: [&str; 5] = ["asdf", "asdf", "asdfadf", "asdf", "asdfdsf"];
    println!("{} {}", b[0], a[1])
}

fn _control_blocks() {
    let mut number = String::new();
    let mut x: u32 = 10;
    // io::stdin().read_line(&mut number);
    match io::stdin().read_line(&mut number) {
        Ok(_) => {
            println!("{}", number);
            x = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => 10,
            };
            println!("x = {}", x);
        }
        Err(_) => {
            println!("invalid number")
        }
    };

    if x < 10 || x == 10 {
        println!("Number is <= 10 {}", x);
    } else if x > 100 {
        println!("Number is >100 {}", x);
    } else {
        println!("Number is {}", x);
    }

    // terinary operator in rust
    println!("x value is {}", if x > 10 { x } else { 10 });
}

fn _loops_in_rust() {
    // loop
    let _x: &str = loop {
        println!("asdfadsf");
        break "10";
    };

    // while
    let mut i = 10;
    while i < 20 {
        println!("{}", i);
        i += 1;
    }
    // for loop
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("The element is:{element}");
    }
    // for loop type 2
    for number in (1..=12).rev() {
        println!("{number}");
    }
    // lyrics of Twelve Days of christmas
    let items_on_each_day: [&str; 12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "a partridge in a pear tree",
    ];
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];
    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[day]
        );
        for i in (1..(day + 1)).rev() {
            println!("{}", items_on_each_day[12 - i - 1]);
        }
        if day > 1 {
            println!("And {}", items_on_each_day[12 - 1]);
        } else {
            println!("{}", items_on_each_day[12 - 1]);
        }
    }
    println!("And {}", items_on_each_day[12 - 1]);
}

// ownership
/**
 *  All data stored on the stack must have a known, fixed size.
 *  Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
 */
fn _ownership() {
    let s1 = String::from("asdfafd");
    let s2 = s1;
    /*
     this step results in a compile time error as ownership is transfered to s2 from s1
     println!("{}", s1);
    */
    let mut s3 = s2.clone(); // this is the way to copy the data into other variable
    println!("{} {}", s2, s3);

    // borrowing
    let length: usize = _calculate_length_2(&mut s3);
    println!("{} {}", length, s3);
    // println!("{}", get_slice(s3));
    // types of borrowing mut variable
    let r1 = &s3;
    println!("s3 value is ({}) borrowed to r1 ({})", s3, r1);
    /*
        let mut r2 = &s3; // this statement won't work as reference of a variable cannot be given to the mut variable
        let r3 = &mut s3; // this statement also won't work
    */
    let r2 = &mut s3;
    println!("{}", r2);
    _borrowing_working_cases();
}

fn _calculate_length(s: String) -> usize {
    /*
    The below statement won't work because the parameter does not has mut
    s.push_str(",asdfasdf");
     */
    s.len()
}

fn _calculate_length_2(s: &mut String) -> usize {
    s.push_str("asdfasdf");
    return s.len();
}

fn _borrowing_working_cases() {
    let mut s3 = String::from("asdfasdfasdf");
    let s2 = &mut s3;
    s2.push_str("asdfasdf");
    let s1 = s2;
    println!("s2 is {}", s1);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn is_user_active(&self) -> bool {
        self.active
    }
    fn get_email(&self) -> &String {
        &self.email
    }
    fn get_username(&self) -> &String {
        &self.username
    }
    fn get_sign_in_count(&self) -> u64 {
        self.sign_in_count
    }
    fn set_email(&mut self) -> &mut String {
        &mut self.email
    }
}

fn compound_datatypes() {
    let user = User {
        active: true,
        username: String::from("Raja Sekhar"),
        email: String::from("raja02sekhar08@gmail.com"),
        sign_in_count: 1,
    };
    let mut user2: User = User {
        active: false,
        ..user
    };
    println!("{:#?} {}", user2, user2.is_user_active());
    *user2.set_email() = String::from("rajasekahr@masdfasdf.com");
    dbg!(
        // user2.set_email(string1),
        user2.get_email(),
        user2.get_username(),
        user2.get_sign_in_count()
    );
}
