extern crate commandMacro;

use std::env::{args,Args};
use std::process::ExitCode;
use commandMacro::{command,command_list};

struct Command{
    name: &'static str,
    description: &'static str,
    run: fn(&str, Args)-> ExitCode
}

fn usage(program: &str) {
    eprintln!("Usage: {program} <command>");
    eprintln!("Commands:");
    for command in COMMANDS.iter() {
        eprintln!("    {name} - {description}", name = command.name, description = command.description);
    }
}

#[command("hello","print hello")]
fn hello_command(_program:&str, args:Args)-> ExitCode{
    println!("Hello, World");
    //println!("hello world {:?}",args.next().expect("Missing 3rd Arg"));
    ExitCode::SUCCESS
}

#[command("help", "Print this help messsage")]
fn help_command(program: &str,mut args: Args) -> ExitCode {
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            println!("{name} - {description}", name = command.name, description = command.description);
        } else {
            eprintln!("ERROR: command {command_name} is not found");
            return ExitCode::FAILURE;
        }
    } else {
        usage(&program);
    }
    ExitCode::SUCCESS
}
const COMMANDS:&[Command] = command_list!();

fn main() -> ExitCode{
    let mut argsIter = args();
    let program_name = argsIter.next().expect("Program Name");
    let cmd = argsIter.next().expect("command Name");

    if let Some(details) = COMMANDS.iter().find(|x| x.name == cmd){
        (details.run)(&program_name, argsIter);
    }else{
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

