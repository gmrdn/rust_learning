fn main() {
    println!("Hello, world!");
    let tf = 68;
    let tc = convert_fahrenheit_to_celsius(tf);
    println!("Converted {} Fahrenheit to {} Celsius", tf, tc);
}

fn convert_fahrenheit_to_celsius(tf: i32) -> i32 {
    let tc = (tf - 32) * 5/9;
    tc
}