pub fn vectors() {
    let mut v: Vec<usize> = Vec::new();
    let v_str: Vec<&str> = vec!["asdf", "asdf", "asdf"];
    println!("{:?}", v_str);
    for (index, el) in v_str.iter().enumerate() {
        v.push(index);
        println!("{} {}", el, v[index]);
        let s = index.to_string();
        match v_str.get(index_reader(&s)) {
            Some(_) => println!("{}", index),
            None => println!("The is not third element"),
        }
        println!("{}", index);
    }
}

pub fn index_reader(index: &String) -> usize {
    let x = match index.trim().parse() {
        Ok(value: u32) => (value + 1),
        _ => 10 as usize,
    };

    x
}
