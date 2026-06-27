extern crate proc_macro;
use std::env::{args,Args};
use std::process::ExitCode;

struct Command{
    name: &'static str,
    des: &'static str,
    run: fn(&str,&mut Args)
}

const COMMANDS:&[Command] = &[
    Command {
        name:"hello",
        des :" print hello",
        run : hello_command
    },
    Command {
        name:"help",
        des :" print help",
        run : help_command
    }
];

/**
    Source - https://stackoverflow.com/a/47515540
    These two are equivalent
    fn map<U>(self, f: impl FnOnce(T) -> U) -> Option<U>
    fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U
 */
fn hello_command(_program:&str, _args:&mut impl Iterator<Item = String>){
    println!("hello world {:?}",_args.next().expect("Missing 3rd Arg"));
}

fn help_command(_program:&str, _args:&mut impl Iterator<Item = String>){
    println!("help Command {:?}",_args.next().expect("Missing 3rd Arg"));
}



fn main() -> ExitCode{
    let mut argsIter = args();
    let program_name = argsIter.next().expect("Program Name");
    let cmd = argsIter.next().expect("command Name");

    if let Some(details) = COMMANDS.iter().find(|x| x.name == cmd){
       /**
        * let fun = details.run;
        * fun(&program_name);
        * shoter version (details.run)(&program_name)
        **/
        (details.run)(&program_name,&mut argsIter);
    }else{
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
