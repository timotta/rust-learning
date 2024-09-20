fn main() {
    let mut s1 = "Hello".to_string();

    s1.push_str(" World");

    println!("S1: {s1}");

    let s2 = "!!!!!!!!!!!".to_string();

    let s3 = s1 + &s2;

    println!("S3: {s3}");

    println!("S2: {s2}");

    let s4 = format!("====> {s3}");

    println!("{s4}");

    for c in s4.chars() {
        println!("Char {c}");
    }
}
