use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("You're on the list.  Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("Management has this note for you:");
                println!("{}", note);
                if self.age < 5 {
                    println!("Don't give root beer to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("Yo, you're on probation..."),
            VisitorAction::Refuse => println!("Go away!! {}", self.name),
        }
    }
}

impl PartialEq for Visitor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
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
        Visitor::new("echo", VisitorAction::Accept, 7),
        Visitor::new("scott", VisitorAction::AcceptWithNote { note: String::from("He likes BBQ sauce") }, 32),
        Visitor::new("snoopy", VisitorAction::AcceptWithNote { note: String::from("Watch out for this guy...")}, 42),
        Visitor::new("woodstock", VisitorAction::Refuse, 12),
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
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
