fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(60, 'm');

    let x = plus_two(7 - 3);
    println!("x = {x}");
}

fn another_function(x: i32) {
    println!("Value of x = {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_two(x: i32) -> i32 {
    x + 2
}