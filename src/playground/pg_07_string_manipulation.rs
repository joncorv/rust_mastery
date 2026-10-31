pub fn string_manipulation() {
    let string_1 = "This is a string".to_string();
    let string_2 = String::from_iter(['t', 'h', 'i', 's', ' ', 'i', 's', ' ', 'a', ' ', 's', 't', 'r', 'i', 'n', 'g']);

    let string_3: Vec<&str> = "This is a string".split(' ').collect();
    let mut string_4 = string_3.clone();
    string_4.reverse();

    println!("string_1 {string_1}");
    println!("string_2 {string_2}");
    println!("string_3 {string_3:?}");
    println!("string_4 {string_4:?}");
}
