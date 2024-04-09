use std::collections::HashMap;

enum Command {
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
    Help,
}

impl Command {
    fn parse_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.split_whitespace().collect();

        match words[..] {
            ["help"] => Some(Command::Help),
            ["quit"] => Some(Command::Quit),
            ["list", dept] => Some(Command::List(dept.to_string())),
            ["all"] => Some(Command::All),
            ["add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}

struct Map {
    company: HashMap<String, Vec<String>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            company: HashMap::new(),
        }
    }

    fn add_employee(&mut self, dept: String, name: String) {
        self.company
            .entry(dept)
            .or_default()
            .push(name);
    }

    fn list(&self, dept: &str) {
        match self.company.get(dept) {
            Some(names) => {
                println!("{dept}: {names:?}");
            },
            None => println!("I don't recognize that department!"),
        }
    }

    fn list_all(&self) {
        self.company
            .iter()
            .for_each(|(dept, names)| {
                println!("{dept}: {names:?}");
            });
    }
}

fn help() {
    println!("Supported commands:");
    println!("1. Add [name] to [department]");
    println!("2. List [department], All, or Quit");
    println!("3. All");
    println!("4. Quit");
    println!("5. Help to bring up this again.\n");
}

fn main() {
    let mut map = Map::new();

    help();
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match Command::parse_input(input.to_lowercase().trim()) {
            Some(Command::Help) => help(),
            Some(Command::Add { dept, name }) => map.add_employee(dept, name),
            Some(Command::List(dept)) => map.list(&dept),
            Some(Command::All) => map.list_all(),
            Some(Command::Quit) => {
                println!("Quitting...");
                break;
            },
            None => println!("Invalid input"),
        }
    }
}
