use std::io;
fn main() {
    let mut age: i8 = 22;
    let is_next_year: bool = true;
    let name: &str = "Jarib";

    let mut personal_details: (&str, i8, bool) = (name, age, is_next_year);
    let mut ages: [i8; 3] = [22, 23, 25];

    // println!(
    //     "personal detail.\nI am {}.\nI am {} years old on {}",
    //     personal_details.0, personal_details.1, personal_details.2
    // );

    if is_next_year == true {
        age = age + 1;
        ages[0] = age;
        personal_details.1 = age;
    }
    // println!(
    //     "personal detail.\nI am {}.\nI am {} years old on {}",
    //     personal_details.0, personal_details.1, personal_details.2
    // );
    // input_functions();
    arithmetic_functions();
}

fn input_functions() {
    // Getting user inputs with rust
    let mut input = String::new();
    println!("What do you think?");
    io::stdin().read_line(&mut input).expect("Failed to read");
    println!("This is what Jarib said: {}", input);
}

fn arithmetic_functions() {
    // only variables of the same data type can interact with each other
    // integers with a definite byte size can't accept 
    // values that are larger than their byte size
    let age_1: u8 = 23;
    let age_2: u8 = 23;

    let _age_3 = age_1 + age_2;
    println!("new age {_age_3}");

    let x: f32 = 2400.0;
    let y: f32 = 23.0;

    let z: f32 = x / y;
    let a: f32 = x % y;
    let b: f32 = x * y;
    let  c = 34 as i16;
    let d:i32 = 255;
    let e = d * (c as i32);
    let max_int = i16::MAX;
    let min_float = f32::MIN;

    println!(" x / y: {z}");
    println!("x % y: {a}");
    println!("x * y: {b}");
    println!("d * (c as i32): {e}");
}
