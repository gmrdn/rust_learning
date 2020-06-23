fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    with_while();
    with_for();
}

fn with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}


fn with_for() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is : {}", element);
    }
}