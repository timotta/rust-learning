enum Message {
    Quit,
    Send { text: String },
    Delete { id: u32 },
    Other
}

impl Message {
    fn execute(&self) {
        match self {
            Message::Send { text } => println!("Sent {text}"),
            Message::Delete { id } => println!("Delete {id}"),
            _ => println!("not implemented yet"),
        }
    }
}

fn main() {
    let q = Message::Quit;
    let s = Message::Send {
        text: "ssssss".to_string(),
    };
    let d = Message::Delete { id: 123 };
    let o = Message::Other;

    q.execute();
    s.execute();
    d.execute();
    o.execute();
}
