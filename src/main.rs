use std::io::{self, Read};
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
    // arithmetic_functions();
    // converting_types();
    // operation_functions();
    let mut is_continue: bool = true;
    let mut is_continue_str = String::new();
    let mut rand_number = String::new();
    while is_continue == true {
        println!("What number would you like to check if it's prime?");
        io::stdin()
            .read_line(&mut rand_number)
            .expect("couldn't read line");
        let test_number = rand_number.trim().parse::<u64>().unwrap();
        let is_prime = is_prime_number(test_number);

        if is_prime == true {
            println!("{test_number} is prime");
        } else {
            println!("{test_number} is not prime");
        }
        println!("Do you want to continue?");
        io::stdin()
            .read_line(&mut is_continue_str)
            .expect("couldn't read line");
        match is_continue_str.trim() {
            "yes" | "y" => is_continue = true,
            "no" | "n" => is_continue = false,
            _ => is_continue = is_continue,
        }
    }
    println!("Completed");
}

fn input_functions() {
    // Getting user inputs with rust
    //
    //
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
    let c = 34 as i16;
    let d: i32 = 255;
    let e = d * (c as i32);
    let max_int = i16::MAX;
    let min_float = f32::MIN;

    println!(" x / y: {z}");
    println!("x % y: {a}");
    println!("x * y: {b}");
    println!("d * (c as i32): {e}");
}

fn converting_types() {
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Couldn't read line");

    let age = age_input.trim().parse::<u16>().unwrap();
    println!("You are {age} years young");
}

fn operation_functions() {
    // >, >=, <, <=, !=, ==, &&, ||, !
    let food: &str = "Fruit";
    match food {
        "Cookies" => println!("I like cookies"),
        "Fruit" => println!("I love fruits"),
        _ => println!("I don't know what that is"),
    };

    if food == "Cookies" {
        println!("I like cookies");
    } else if food == "Fruit" {
        println!("I love fruits");
    } else {
        println!("I don't know what that is");
    }
}

fn is_prime_number(value: u64) -> bool {
    for n in 2..=value {
        if value % n == 0 {
            return false;
        }
    }
    return true;
}
