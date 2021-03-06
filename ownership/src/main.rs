fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    let mut s4 = String::from("hello mutation");
    change(&mut s4);
}

fn change(some_string: &mut String) {
    some_string.push_str(" land");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


