fn main() {
    // Parses "42" to u32
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // INT
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{y}");

    // mathematical operations
    // addition
    let sum = 5 + 10;
    println!("{sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    // Boolean
    let t = true;
    let f: bool = false;
    println!("{f}");

    // Character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    // Tuple -> Different types allowed
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{six_point_four}");

    // Array -> Only same types. Fixed Length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{second}");

}