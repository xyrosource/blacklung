extern crate rustyline;

pub mod errors {
    error_chain!{}
}

use self::errors::*;

enum Command {
    Quit,
    Noop
}

pub fn start() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline("λ ");

        use self::rustyline::error::ReadlineError;
        let command = match readline {
            Ok(ref line) => {
                rl.add_history_entry(line);
                parse_command(line)
            },
            Err(ReadlineError::Eof) => break,
            Err(_) => continue,
        };

        match command {
            Ok(Command::Quit) => break,
            Ok(Command::Noop) => continue,
            Err(msg) => println!("{}", msg),
        }
    }

    Ok(())
}

fn parse_command(line: &str) -> Result<Command> {
    match line.trim() {
        "quit" | "exit" => Ok(Command::Quit),
        "" => Ok(Command::Noop),
        _ => bail!("'{}' is not a valid command", line),
    }
}
