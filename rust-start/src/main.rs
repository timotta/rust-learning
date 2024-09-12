use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("AAAAAAAAAAAAAAAAAAAAA");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout);
    say(&message, width, &mut writer).unwrap();
}
