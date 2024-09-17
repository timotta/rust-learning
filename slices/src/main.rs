fn main() {
    let message = String::from("Incredible Message Here");

    let word1 = n_word(&message, 1);
    let word2 = n_word(&message, 2);
    let word3 = n_word(&message, 3);
    let word4 = n_word(&message, 4);

    println!("First word: [{word1}]");
    println!("Second word: [{word2}]");
    println!("Third word: [{word3}]");
    println!("Fourth word: [{word4}]");
}

fn n_word(s: &str, position: usize) -> &str {
    let bytes = s.as_bytes();

    let mut last_space = 0;
    let mut word_position: usize = 0;

    for (i, &val) in bytes.iter().enumerate() {
        if val == b' ' {
            if word_position == position - 1 {
                return &s[last_space..i];
            }
            word_position += 1;
            last_space = i+1;
        }
    }

    if word_position == position - 1 {
        return &s[last_space..];
    }
    ""
}
