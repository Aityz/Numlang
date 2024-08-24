#[derive(PartialEq)]
pub enum Keyword {
    Trans,   // translate Brainfuck to Numlang
    Run,     // Run the Numlang/Brainfuck using the Interpreter
    Compile, // Compile the Numlang/Brainfuck to Rust
    Hex,
    None,
}

pub struct Args {
    pub keyword: Keyword,

    pub file: String,

    pub help: bool,
    pub version: bool,

    pub brainfuck: bool, // without this, Numlang will interpret the file as numlang
}

impl Args {
    pub fn new() -> Self {
        let mut args = std::env::args().collect::<Vec<String>>();

        args.remove(0);

        let mut kw = Keyword::None;
        let mut file = String::new();

        let mut help = false;
        let mut version = false;
        let mut brainfuck = false;

        for arg in args {
            match arg.to_lowercase().as_str() {
                "trans" | "translate" => kw = Keyword::Trans,
                "run" => kw = Keyword::Run,
                "comp" | "compile" => kw = Keyword::Compile,
                "hex" | "hexa" => kw = Keyword::Hex,

                "--help" | "-h" => help = true,
                "--version" | "-v" => version = true,
                "--brainfuck" | "-bf" | "-b" => brainfuck = true,

                _ => file = arg,
            }
        }

        Args {
            keyword: kw,
            file,
            help,
            version,
            brainfuck,
        }
    }
}
