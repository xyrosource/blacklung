// Copyright 2017 The Xyrosource Team.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustyline;

#[derive(Debug)]
pub enum Command {
    Dummy,
}

pub struct Console {
    editor: rustyline::Editor<()>,
    error: bool,
}

impl Console {
    pub fn new() -> Self {
        Console {
            editor: rustyline::Editor::<()>::new(),
            error: false,
        }
    }
}

impl Iterator for Console {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let readline = self.editor.readline(if self.error { "! " } else { "λ " });
            use self::rustyline::error::ReadlineError;
            match readline {
                Ok(ref line) => {
                    match line.into() {
                        InternalCommand::Empty => continue,
                        InternalCommand::Invalid => {
                            self.error = true;
                            self.editor.add_history_entry(line);
                            continue;
                        }
                        InternalCommand::Entry(cmd) => {
                            self.error = false;
                            self.editor.add_history_entry(line);
                            return Some(cmd);
                        }
                        InternalCommand::Quit => return None,
                        InternalCommand::Help => {
                            self.error = false;
                            self.editor.add_history_entry(line);
                            display_help();
                            continue;
                        }
                    }
                }
                Err(ReadlineError::Eof) => return None,
                Err(_) => continue,
            };
        }
    }
}

enum InternalCommand {
    Entry(Command),
    Empty,
    Invalid,
    Quit,
    Help,
}

impl<T: AsRef<str>> From<T> for InternalCommand {
    fn from(other: T) -> Self {
        match other.as_ref().trim() {
            "" => InternalCommand::Empty,
            "quit" | "exit" => InternalCommand::Quit,
            "dummy" => InternalCommand::Entry(Command::Dummy),
            "help" => InternalCommand::Help,
            _ => InternalCommand::Invalid,
        }
    }
}

fn display_help() {
    println!("\
help  - Display help
quit  - Exit the server
exit  - Exit the server
dummy - \
              Example command
");
}
