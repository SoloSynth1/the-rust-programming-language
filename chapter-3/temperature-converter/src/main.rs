use std::io;

fn c2f(input: f32) -> f32 {
    input * 1.8 + 32.0
}

fn f2c(input: f32) -> f32 {
    (input - 32.0) / 1.8
}

fn read_input(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Failed to read line");
}

fn main() {
    let mut input = String::new();

    println!("Please enter the conversion mode (`c` or `f`)");
    read_input(&mut input);

    let mode: char = input
        .trim()
        .parse()
        .expect("Input must be a character");

    let convert_fn: fn(f32) -> f32;
    let input_name: &str;
    let output_name: &str;

    if mode == 'c' {
        convert_fn = c2f;
        input_name = "Celsius";
        output_name = "Fahrenheit";
    } else if mode == 'f' {
        convert_fn = f2c;
        input_name = "Fahrenheit";
        output_name = "Celsius";
    } else {
        panic!("Invalid conversion mode");
    }

    println!("Please enter the temperature in {input_name}:");
    read_input(&mut input);

    let input_temp: f32 = input
        .trim()
        .parse()
        .expect("Input must be a number");

    let result = convert_fn(input_temp);

    println!("The converted temperate is {result} degrees {output_name}");
}
