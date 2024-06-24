use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Visitor {
            name: name.trim().to_lowercase(),
            greeting: greeting.trim().to_string(),
        }
    }

    fn greet(&self) -> () {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");

    let name = what_is_your_name();
    let visitor_list = [
        Visitor::new("Fred", "Hello Fred"),
        Visitor::new("Alice", "Who the fxxk is Alice?"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(v) => v.greet(),
        None => println!("Don't know you!"),
    }
}
