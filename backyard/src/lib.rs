mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist()
}
