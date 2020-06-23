fn main() {
    let x = five();
    println!("Hello, world!");
    another_function(x, 6);

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}