fn main() {
    println!("Hello, world!");
    println!("first word: {}", first_word(&String::from("Lorem ipsum et blabla bla etc.")));
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
