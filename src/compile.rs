pub fn compile_bf(f: String) {
    if crate::translater::translate(f.clone(), true).is_ok() {
        let mut parts = f.split('.').collect::<Vec<&str>>();

        parts[1] = "rs";

        std::process::Command::new("rustc")
            .arg(&parts.join("."))
            .spawn()
            .unwrap_or_else(|_| {
                println!("error: failed to start rustc");
                std::process::exit(1);
            })
            .wait()
            .unwrap_or_else(|_| {
                println!("error: failed to wait for rustc");
                std::process::exit(1);
            });
    } else {
        println!("error: failed to translate brainfuck to rust");

        std::process::exit(1);
    }
}

pub fn compile(f: String) {
    if crate::translater::translate(f.clone(), false).is_ok() {
        let mut parts = f.split('.').collect::<Vec<&str>>();

        parts[1] = "rs";

        std::process::Command::new("rustc")
            .arg(&parts.join("."))
            .spawn()
            .unwrap_or_else(|_| {
                println!("error: failed to start rustc");
                std::process::exit(1);
            })
            .wait()
            .unwrap_or_else(|_| {
                println!("error: failed to wait for rustc");
                std::process::exit(1);
            });
    } else {
        println!("error: failed to translate numlang to rust");

        std::process::exit(1);
    }
}
