pub fn bf_to_nl(f: String) -> Result<(), std::io::Error> {
    let mut data = std::fs::read_to_string(&f)?;

    data = data.replace('>', "1");
    data = data.replace('<', "2");

    data = data.replace('+', "3");
    data = data.replace('-', "4");

    data = data.replace('.', "6");
    data = data.replace(',', "5");

    data = data.replace('[', "c");
    data = data.replace(']', "d");

    let mut parts = f.split('.').collect::<Vec<&str>>();

    parts[1] = "nl";

    if parts.join(".") == f {
        println!("error: file already had .nl extension");

        std::process::exit(1);
    } else {
        std::fs::write(parts.join("."), data)?;
    }

    Ok(())
}

pub fn translate(f: String, brainfuck: bool) -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string(&f).unwrap_or_default();

    let base = include_str!("../base.rs");

    // formatting stuff

    let mut lines = base.lines().map(|x| x.to_string()).collect::<Vec<String>>();

    lines.pop();

    lines.push(String::default());

    let mut loop_stack = Vec::new();
    let mut loop_count = 0;

    if brainfuck {
        for char in data.chars() {
            match char {
                '+' => {
                    lines.push("\tinc_val(&mut stack, &ptr);".to_string());
                }

                '-' => {
                    lines.push("\tdec_val(&mut stack, &ptr);".to_string());
                }

                '>' => {
                    lines.push("\tinc_ptr(&mut ptr);".to_string());
                }

                '<' => {
                    lines.push("\tdec_ptr(&mut ptr);".to_string());
                }

                '.' => {
                    lines.push("\tprint_byte(&mut stack, &ptr);".to_string());
                }

                ',' => {
                    lines.push("\tread_byte(&mut stack, &ptr);".to_string());
                }

                '[' => {
                    loop_count += 1;

                    loop_stack.push(loop_count);

                    lines.push("\twhile stack[ptr] != 0 {".to_string());
                }

                ']' => {
                    if loop_stack.pop().is_some() {
                        lines.push("\t}".to_string());
                    } else {
                        return Err(std::io::ErrorKind::Other.into());
                    }
                }

                _ => (),
            }
        }
    } else {
        for char in data.chars() {
            match char {
                '1' => {
                    lines.push("\tinc_ptr(&mut ptr);".to_string());
                }

                '2' => {
                    lines.push("\tdec_ptr(&mut ptr);".to_string());
                }

                '3' => {
                    lines.push("\tinc_val(&mut stack, &ptr);".to_string());
                }

                '4' => {
                    lines.push("\tdec_val(&mut stack, &ptr);".to_string());
                }

                '5' => {
                    lines.push("\tread_byte(&mut stack, &ptr);".to_string());
                }

                '6' => {
                    lines.push("\tprint_byte(&mut stack, &ptr);".to_string());
                }

                '7' => {
                    lines.push("\tprint_user_stack(&mut user_stack);".to_string());
                }

                '8' => {
                    lines.push("\tappend_to_us(&mut stack, &ptr, &mut us);".to_string());
                }

                '9' => {
                    lines.push("\tremove_newest_from_us(&mut user_stack);".to_string());
                }

                'a' => {
                    lines.push("\tclear_stack(&mut user_stack);".to_string());
                }

                'b' => {
                    lines.push("\twrite_len(&mut stack, &ptr, &mut user_stack".to_string());
                }

                'c' => {
                    loop_count += 1;

                    loop_stack.push(loop_count);

                    lines.push("\twhile stack[ptr] != 0 {".to_string());
                }

                'd' => {
                    if loop_stack.pop().is_some() {
                        lines.push("\t}".to_string());
                    } else {
                        return Err(std::io::ErrorKind::Other.into());
                    }
                }

                _ => (),
            }
        }
    }

    lines.push("".to_string());

    lines.push("\tclose()".to_string());

    lines.push("}".into());

    let mut parts = f.split('.').collect::<Vec<&str>>();

    parts[1] = "rs";

    if f == parts.join(".") {
        println!("error: file already had .rs extension");

        std::process::exit(1);
    } else {
        std::fs::write(parts.join("."), lines.join("\n"))?;
    }

    Ok(())
}
