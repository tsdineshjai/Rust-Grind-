fn main() {
    println!("{}", is_even(22));

    println!("{}", process_string("Hello World".to_string()));
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}

fn process_string(input: String) -> String {
    input.to_lowercase()
}

