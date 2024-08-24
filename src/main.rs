//! Num lang, a funny esoteric programming language like brainfuck
//!
//! # Basic Operations
//! 1 -> Move pointer right by one
//! 2 -> Move pointer left by one
//! 3 -> Increment value by one
//! 4 -> Decrement value by one
//!
//! # File I/O
//! 5 -> Read byte from stdin to pointer location
//! 6 -> Print current byte to stdout
//! 7 -> Print bytes in stack to stdout
//!
//! # Stack Manipulation
//! 8 -> Add current byte to stack
//! 9 -> Remove newest item from stack
//! a -> Clear stack
//! b -> Write stack length to current byte
//!
//! # Looping
//! c -> Open loop
//! d -> Close loop

mod args;
mod compile;
mod interpreter;
mod translater;

fn main() {
    let args = args::Args::new();

    if args.help {
        println!("Num-lang compiler:\n");
        println!("Usage: numlang [options] [command] [file]\n");
        println!("Options:");
        println!("  --help, -h:            Show help");
        println!("  --version, -v:         Show version");
        println!("  --bf, -b, --brainfuck: Compile brainfuck instead of numlang\n");
        println!("  --ignore, -i:          Ignore loops while using Hex");
        println!("Commands:");
        println!("  run                    Run Brainfuck/numlang code with the interpreter");
        println!("  compile                Compile Brainfuck/numlang to a binary");
        println!("  trans                  Transpile BF to NL or NL/BF to Rust");
        println!("  hex                    Run a file as Numlang");

        std::process::exit(0);
    } else if args.version {
        println!("Num-lang compiler 0.1.0");

        std::process::exit(0);
    }

    if args.keyword == args::Keyword::None {
        println!("error: no keyword supplied");

        std::process::exit(1);
    } else if !std::path::PathBuf::from(&args.file).exists() {
        println!("error: file does not exist");
    } else {
        match args.keyword {
            args::Keyword::Run => {
                if args.brainfuck {
                    if let Err(e) = interpreter::run_bf(args.file) {
                        println!("error: {:?}", e)
                    };
                } else {
                    interpreter::run(args.file);
                }
            }

            args::Keyword::Compile => {
                if args.brainfuck {
                    compile::compile_bf(args.file)
                } else {
                    compile::compile(args.file)
                }
            }

            args::Keyword::Trans => {
                if args.brainfuck {
                    if let Err(e) = translater::bf_to_nl(args.file) {
                        println!("error: {:?}", e);
                    }
                } else if let Err(e) = translater::translate(args.file, false) {
                    println!("error: {:?}", e);
                }
            }

            args::Keyword::Hex => {
                // hex will always be numlang

                let mut parts = args.file.split('.').collect::<Vec<&str>>();

                if parts.len() != 1 {
                    parts[1] = "nl";
                } else {
                    parts.push("nl");
                }

                if parts.join(".") == args.file {
                    parts[1] = "neolang";
                }

                let filename = parts.join(".");

                let data = std::fs::read_to_string(args.file).unwrap_or_default();

                let mut file_data = String::new();

                for char in data.chars() {
                    file_data.push_str(format!("{:02x}", (char as u8)).as_str());
                }

                if args.ignore {
                    file_data = file_data.replace('c', "");

                    file_data = file_data.replace('d', "");
                }

                if std::fs::write(&filename, file_data).is_err() {
                    println!("error: failed to writ to {}", filename);
                }
            }

            args::Keyword::None => unreachable!(),
        }
    }
}
