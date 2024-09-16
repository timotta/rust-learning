fn main() {
    let sumed = crazy_math(10, 20);
    let subtracted = crazy_math(100, 10);
    let multiplied = crazy_math(8, 8);

    println!("The sum is {sumed} {subtracted} {multiplied}");

    let fib_until = fib_loop_until(30);

    println!("Fib {fib_until}");

    let mut fib_vec = fib_loop_vec(10);
    print_vec(&fib_vec);

    fib_vec.reverse();
    print_vec(&fib_vec);
}

fn print_vec(vec: &[i32; 100]) {
    println!("----------------------");
    for i in vec {
        if *i != 0 {
            println!("- fib {i}")
        }
    }
}

fn crazy_math(a: i32, b: i32) -> i32 {
    if b < a {
        a - b
    } else if a == b {
        a * b
    } else {
        a + b
    }
}

fn fib_loop_until(max: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    loop {
        let tmp = b;
        if a + b >= max {
            break b;
        }
        b = a + b;
        a = tmp;
    }
}


fn fib_loop_vec(max_index: i32) -> [i32; 100] {
    let mut result = [0; 100];

    let mut index: i32 = 0;
    let mut a = 1;
    let mut b = 1;

    while index <= max_index-1 {
        let tmp = b;
        b = a + b;
        a = tmp;
        result[ index as usize ] = b;
        index += 1;
    }

    return result;
}




