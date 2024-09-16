fn main() {
    let a = String::from("XPTO");

    let a = takes_ownership(a);

    println!("Got back ownership {a}");

    let size = calculate_length(&a);

    println!("LEN {size}");

    let b = &a;

    println!("B = {b}");

    println!("A = {a}");

    let mut x = String::from("Prefix");

    println!("x before = {x}");

    change(&mut x, ": suffix");

    println!("x after = {x}");
}

fn takes_ownership(a: String) -> String {
    println!("Took {a}");
    a
}

fn calculate_length(a: &String) -> usize {
    a.len()
}

fn change(original: &mut String, toappend: &str) {
    original.push_str(&toappend);
}