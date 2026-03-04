use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn prompt_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read name");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = vec![
        Visitor::new("echo", "Hello, Echo!"),
        Visitor::new("scott", "Hola, Scott!"),
        Visitor::new("snoopy", "What's up snoop dog?"),
    ];

    println!("Hello, what is your name?");
    let name = prompt_name();

    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Go away!! {}", name)
    }
}
