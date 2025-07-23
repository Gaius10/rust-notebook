use crate::database::Database;

#[derive(Debug)]
pub enum Command {
    List,
    Exit,
    Error(String),
    Add {
        employee: String,
        group: String,
    },
}

impl Command {
    pub fn from(s: String) -> Self {
        make_from_string(s)
    }

    pub fn exec(self, database: &mut Database) {
        match self {
            Command::List => database.list(),
            Command::Add  { employee, group } => {
                database.add(employee, group);
            },
            Command::Exit => std::process::exit(0),
            Command::Error(m) => println!("Error: {m}."),
        }
    }
}

pub fn make_from_string(s: String) -> Command {
    let mut words = s.split_whitespace();
    let first_word = words.next();

    match first_word {
        None => Command::Error(String::from("command expected")),
        Some("Exit") => Command::Exit,
        Some("List") => Command::List,
        Some("Add") => {
            let Some(employee) = words.next() else {
                return Command::Error(
                    String::from("couldn't parse command line")
                )
            };
            words.next();
            let Some(group) = words.next() else {
                return Command::Error(
                    String::from("couldn't parse command line")
                )
            };

            Command::Add {
                employee: employee.to_string(),
                group: group.to_string(),
            }
        },
        Some(unknown) => {
            Command::Error(format!("{unknown} is not a command"))
        }
    }
}

