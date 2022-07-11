pub fn ownership_main_1() {
    let s1: String = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{} {}", s1, s3);
    ownership_main_2();
    mutable_and_immutable_share_of_same_variable();
}

pub fn gives_ownership() -> String {
    let s1 = String::from("arajse arkhar");
    s1
}

pub fn takes_and_gives_back(s1: String) -> String {
    s1
}

pub fn ownership_main_2() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {} is {}", s2, len);
}

pub fn calculate_length(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}
/*
    why referencing ?
    Because without referencing ownership is tranfered to the calling function, assigned variables etc
    with reference we are giving a immutable object to the calling function, assigned variables which will prevent from modifying the content of the given item
    with mut reference it is very clear that the change function will mutate the value it borrows. Mutable reference will change the value of the actual object
*/
pub fn reference_main() {
    let mut s1 = String::from("hello");
    change_str(&mut s1);
}

pub fn change_str(s1: &mut String) {
    s1.push_str("asdfasdf");
}

pub fn mutable_and_immutable_share_of_same_variable() {
    let mut s = String::from("hello");
    println!("{}", s[..].len());
    let r1 = &s; // no problem
    let r2 = &s; /* no problem*/
    /*
        let r3 = &mut s; // Big Problem because r1,r2 are the immutable reference to the same variable s and those references are used after this statement
    */
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
