use std::io;
use company::command::Command;
use company::database::Database;

/**
 * Available commands:
 *
 * - List
 * - Add <name> to <group>
 *
 */

fn main() {
    let mut database = Database::new();    

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input.");

        let command = Command::from(input);
        command.exec(&mut database);
    }
}

