use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

fn main() {
    let mut my_vect: Vec<i8> = vec![10, 20, 30];

    for i in &my_vect {
        println!("{i}");
    }

    found_not_found(&my_vect, 3);
    found_not_found(&my_vect, 2);

    my_vect.push(40);

    found_not_found(&my_vect, 3);

    let x = &my_vect[2];

    println!("X {x}");

    my_vect.push(50);

    //println!("X {x}");






    //println!("X {x}"); // Error if uncommenting because x will still be borrowing

    found_not_found(&my_vect, 4);

    let mut scores = HashMap::new();

    scores.insert("blue".to_string(), 10);
    scores.insert("green".to_string(), 20);

    let score_blue = scores.get(&"blue".to_string()).copied().unwrap_or(0);
    let score_red = scores.get(&"red".to_string()).copied().unwrap_or(0);

    println!("Score blue {score_blue}");
    println!("Score red {score_red}");

    scores.entry(String::from("red")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(999999);

    println!("Registereds:");

    for (k, v) in &scores {
        println!("- {k} = {v}")
    }

    let mut word_count = HashMap::new();
    let my_text = "my text is a good text to read text";
    for word in my_text.split_whitespace() {
        let value = word_count.entry(word).or_insert(0);
        *value += 1;
    }

    let w = word_count.get("text").copied().unwrap_or(-1);
    println!("W: {w}");

    println!("Word count:");

    for (k, v) in &word_count {
        println!("- {k} = {v}")
    }

    println!("{word_count:?}");

    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    quertion_one();
    question_two();
}

fn found_not_found(my_vect: &Vec<i8>, index: usize) {
    let x = my_vect.get(index);
    if let Some(i) = x {
        println!("Found {index} with value {i}")
    } else {
        println!("Not found {index}")
    }
}

// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.
fn quertion_one() {
    assert_eq!(4.0, median(vec![7, 6, 1, 2, 4, 3, 5]));
    assert_eq!(3.5, median(vec![1, 2, 3, 4, 5, 6]));
    println!("Question one OK!");
}

fn median(mut v1: Vec<i32>) -> f32 {
    v1.sort();
    let is_even = v1.len() % 2 == 0;
    return if is_even {
        let middle = v1.len() / 2;
        (v1[middle - 1] + v1[middle]) as f32 / 2.0
    } else {
        let middle = (v1.len() + 1) / 2;
        v1[middle - 1] as f32
    };
}

// Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and ay is added, so first becomes irst-fay
// Words that start with a vowel have hay added to the end instead
// (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
fn question_two() {
    assert_eq!(
        "ovô-Vay iu-vay a-hay ulva-vay a-day ovó-vay",
        pig_latin("Vovô viu a vulva da vovó")
    );
    assert_eq!(
        "inha-May adeira-cay aqui-hay",
        pig_latin("Minha cadeira aqui")
    );
}

fn vocals() -> &'static HashSet<char> {
    static HASHMAP: OnceLock<HashSet<char>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        HashSet::from(['a', 'e', 'i', 'o', 'u', 'y'])
    })
}

fn pig_latin(text: &str) -> String {
    let mut result = Vec::new();

    for word in text.split_whitespace() {

        let mut new_word = String::new();

        let first_word = word.to_lowercase().chars().nth(0).unwrap_or(' ');
        if vocals().contains(&first_word) {
            new_word.push_str(word);
            new_word.push_str("-hay");
        } else {
            let sliced_word = slice(word, Some(1), None);
            let first_char = word.chars().nth(0).unwrap_or('h');
            new_word.push_str(sliced_word);
            new_word.push('-');
            new_word.push(first_char);
            new_word.push_str("ay");
        }

        result.push(new_word.to_string());
    }

    result.join(" ")
}


fn slice(text: &str, start: Option<usize>, end: Option<usize>) -> &str {
    if text.len() == 0 {
        return text
    }

    let size = text.len();

    let start = start.unwrap_or(0);
    let end = end.unwrap_or(size);

    let start_index = text.char_indices().nth(start).unwrap_or((size, ' ')).0;
    let end_index = text.char_indices().nth(end).unwrap_or((size, ' ')).0;

    return &text[start_index..end_index]
}


// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company; for example, “Add Sally to Engineering” or
// “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or
// all people in the company by department, sorted alphabetically.
fn question_three() {

    loop {



    }

}