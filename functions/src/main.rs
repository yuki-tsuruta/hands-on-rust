fn main() {
    println!("Hello, world!");
    another_function(2);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is: {}", x);
    let y = plus_one(5);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
