use std::process::ExitCode;

use crate::cli::GreetArgs;

/// `nxus` `greet` command handler.
pub fn greet(args: GreetArgs) -> ExitCode {
    println!("Hello from nxus!");
    if let Some(name) = args.name {
        println!("Hi, {name}");
    }

    ExitCode::SUCCESS
}
