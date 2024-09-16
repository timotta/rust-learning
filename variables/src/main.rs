use std::io;

fn main() {
    let x = 5;
    println!("XXXX {x}");
    let x = 6;
    println!("XXXX {x}");
    let x = x * 2;
    println!("XXXX {x}");
    {
        let x = x+1;
        println!("inside {x}");
    }
    println!("outside {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y , z)  = tup;

    println!("Tuple {x} {y} {z}");

    let mut read = String::new();

    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");

    let read: u8 = read.trim().parse().expect("Incorrect value");

    let other: u8 = read + 1;

    println!("Overflow: {other}");

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
