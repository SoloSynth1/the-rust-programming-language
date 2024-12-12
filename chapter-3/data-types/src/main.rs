use std::io;

fn main() {
    let x = 0.2;
    let y = 0.1;
    let res = x + y;
    println!("{x} + {y} = {res}");

    let intdiv = -5 / 3;
    let floatdiv = -5. / 3.;
    
    println!("{intdiv}, {floatdiv}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
