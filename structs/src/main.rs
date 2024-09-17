struct Person {
    name: String,
    email: String,
}

fn print_person(p: &Person) {
    let name = &p.name;
    let email = &p.email;
    println!("Person name=[{name}] email=[{email}]")
}

fn build_person_1(name: &str, email: &str) -> Person {
    return Person {
        name: name.to_string(),
        email: email.to_string(),
    };
}

fn build_person_2(name: String, email: String) -> Person {
    return Person { name, email };
}

//-----------------------------------------------------------------------------

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn increase_width(&mut self, value: u32) {
        self.width += value;
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let p1 = Person {
        name: "Tiago".to_string(),
        email: "tiago@tiago.com".to_string(),
    };
    print_person(&p1);

    let p2: Person = Person {
        name: "Tiago Motta".to_string(),
        ..p1
    };
    print_person(&p2);

    let p3 = build_person_1("Chapun", "di@for.com");
    print_person(&p3);

    let p4 = build_person_2("Other".to_string(), "other@othe.com".to_string());
    print_person(&p4);

    let mut rect = dbg!(Rect {
        width: dbg!(10 + 1),
        height: 20,
    });

    let area = dbg!(rect.area());
    println!("Rect {rect:?} has area of {area}");

    rect.increase_width(10);

    let area2 = rect.area();
    println!("Rect {rect:?} has area of {area2}");

    let r_small = Rect {
        width: 10,
        height: 10,
    };
    let r_big = Rect {
        width: 12,
        height: 10,
    };

    println!("Small can hold big? {}", r_small.can_hold(&r_big));
    println!("Big can hold small? {}", r_big.can_hold(&r_small));

    dbg!(Rect::square(30));
}
