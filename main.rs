use std::env::args;
use std::process::ExitCode;

trait Command{
    fn name() -> &'static str;
    fn description() -> &'static str;
    fn run();
}

implement Command for HelloCommand{
    fn name() -> &'static str{
        "hello"
    }

    fn description() -> &'static str{
        "print \"hello\" "
    }

    fn run() {
    }
}

fn main() -> ExitCode{
    let mut argsIter = args();
    let program_name = argsIter.next().expect("Program Name");

    print!("Hello");
    ExitCode::FAILURE
}
