fn main() {
    let mut my_vect: Vec<i8> = vec![10, 20, 30];

    /*
    for i in &my_vect {
        println!("{i}");
    }

    found_not_found(&my_vect, 3);
    found_not_found(&my_vect, 2);

    my_vect.push(40);

    found_not_found(&my_vect, 3);
     */

    let x: &i8 = &my_vect[2];

    println!("X {x}");

    my_vect.push(50);

    //println!("X {x}"); // Error if uncommenting because x will still be borrowing

    //found_not_found(&my_vect, 4);
}

fn found_not_found(my_vect: &Vec<i8>, index: usize) {
    let x = my_vect.get(index);
    if let Some(i) = x {
        println!("Found {index} with value {i}")
    } else {
        println!("Not found {index}")
    }
}
