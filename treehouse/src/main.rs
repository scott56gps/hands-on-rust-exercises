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

impl PartialEq for Visitor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
    let mut visitor_list = vec![
        Visitor::new("echo", "Hello, Echo!"),
        Visitor::new("scott", "Hola, Scott!"),
        Visitor::new("snoopy", "What's up snoop dog?"),
    ];

    loop {
        println!("Hello, what is your name? (Leave empty and press ENTER to quit)");
        let name = prompt_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
