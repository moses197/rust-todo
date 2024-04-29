use std::env;
use std::process::exit;

mod command;

use command::{AddCommand, ListCommand};
use crate::command::Command;

fn main() -> () {

    let args: Vec<String> = env::args().collect();

    let command: &String = args.get(1).unwrap_or_else(|| {
        display_help();

        exit(0);
    });

    // dbg!(command);

    let exit_code = match command.as_str() {

        "add" => AddCommand::new(args).handle(),

        "list" => ListCommand::new().handle(),
        
        _ => {
            println!("Unknown command");

            1
        },
    };

    exit(exit_code);
}

fn display_help() -> () {
    println!("Usage: todo <command> <target>");
    
    println!();
    println!("Commands:");
    println!("  add <description> - add a todo");
    println!("  list              - display all todos");
    println!();

}