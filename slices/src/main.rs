fn main() {
    println!("Hello, world!");
    println!("first word: {}", first_word(&String::from("Lorem ipsum et blabla bla etc.")));
    println!("string slice : {}", &String::from("string slice")[0..5]);
}


fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
