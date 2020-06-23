fn main() {
    println!("Hello, world!");
    let tf = 68;
    let tc = convert_fahrenheit_to_celsius(tf);
    println!("Converted {} Fahrenheit to {} Celsius", tf, tc);

    let nth = 16;
    let fibo = generate_nth_fibo(nth);
    println!("Generated {} as the {}th Fibonacci number", fibo, nth);
}

fn convert_fahrenheit_to_celsius(tf: i32) -> i32 {
    let tc = (tf - 32) * 5/9;
    tc
}

fn generate_nth_fibo(n: i32) -> i32 {
    let mut previous_number: i32 = 0;
    let mut current_number: i32 = 1;
    let mut next_number: i32 = 0;
    let mut iteration: i32= 2;
    
    while iteration <= n {
        next_number = previous_number + current_number;
        previous_number = current_number;
        current_number = next_number;        
        iteration += 1;
    }
    return next_number;

}