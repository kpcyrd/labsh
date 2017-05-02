extern crate clap;
extern crate rustyline;

use clap::{App, Arg};

mod config;
mod build;

// TODO: seperate interface for scripting?

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let matches = App::new("lab shell")
                    .about("todo")
                    .arg(Arg::with_name("n")
                        .short("n")
                        .help("Dry run"))
                    .get_matches();

    let dry = match matches.occurrences_of("n") {
        0 => false,
        _ => true,
    };

    let mut rl = Editor::<()>::new();

    let config: config::Config = config::load("build.toml");

    loop {
        let readline = rl.readline(&format!(" labsh@{}% ", config.hostname));

        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue
                }

                rl.add_history_entry(&line);

                let build = match config.get_build(&line) {
                    Some(build) => {
                        build
                    },
                    None => {
                        println!("\x1b[1;31m[!] not found\x1b[0m");
                        continue
                    },
                };

                for task in build.tasks() {
                    println!("\x1b[1;34m[+] starting {}\x1b[0m", task.format());
                    match task.run(dry) {
                        Ok(_) => {
                            println!("\x1b[1;32m[+] success\x1b[0m");
                        },
                        Err(err) => {
                            println!("\x1b[1;31m[-] error: {:?}\x1b[0m", err);
                            println!("\x1b[1;31m[!] aborting build\x1b[0m");
                            break
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                continue
            }
            Err(ReadlineError::Eof) => {
                break
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }
}
