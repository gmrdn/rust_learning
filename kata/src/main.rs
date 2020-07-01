fn main() {
    println!("Hello, world!");
    let bmi = bmi(50, 1.80);
    println!("bmi {}", bmi);
    let bounces = bouncing_ball(3.0, 0.66, 1.5);
    println!("bounces {}", bounces);
    let diff = array_diff(vec![1, 2], vec![1]);
    println!("array diff {:?}", diff);
}

fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = calc_bmi(weight, height);
    get_bmi_conclusion(bmi)
}

fn get_bmi_conclusion(bmi: f32) -> &'static str {
    match bmi {
        b if b <= 18.5 => "Underweight",
        b if b <= 25.0 => "Normal",
        b if b <= 30.0 => "Overweight",
        b if b > 30.0 => "Obese",
        _ => "",
    }
}

fn calc_bmi(weight: u32, height: f32) -> f32 {
    let bmi: f32 = weight as f32 / (height * height);
    bmi
}

fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h < 0.0 {
        return -1;
    }

    if bounce <= 0.0 || bounce >= 1.0 {
        return -1;
    }

    if window >= h {
        return -1;
    }

    let mut nb_views: i32 = 1;

    let mut remaining_h: f64 = h * bounce;
    while remaining_h > window {
        nb_views += 2;
        remaining_h = remaining_h * bounce;
    }
    nb_views
}

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.iter().any(|y| y == x)).collect()
}

#[test]
fn basic_tests() {
    assert_eq!(bmi(50, 1.80), "Underweight");
    assert_eq!(bmi(80, 1.80), "Normal");
    assert_eq!(bmi(90, 1.80), "Overweight");
    assert_eq!(bmi(110, 1.80), "Obese");
}

fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }
}
