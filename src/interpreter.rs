use std::io::{Read, Write};

pub fn run_bf(f: String) -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string(f)?;

    let mut stack: Vec<u8> = vec![0; 30000];

    let mut ptr: usize = 0;

    let mut loop_stack: Vec<usize> = Vec::new();

    let chars: Vec<char> = data.chars().collect();

    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '>' => ptr = (ptr + 1) % 30000,
            '<' => ptr = (ptr + 29999) % 30000,
            '+' => {
                if stack[ptr] == 255 {
                    stack[ptr] = 0;
                } else {
                    stack[ptr] += 1;
                }
            }
            '-' => {
                if stack[ptr] == 0 {
                    stack[ptr] = 255;
                } else {
                    stack[ptr] -= 1;
                }
            }
            ',' => {
                let mut buf = [0; 1];

                std::io::stdin().read_exact(&mut buf)?;

                stack[ptr] = buf[0];
            }
            '.' => {
                print!("{}", stack[ptr] as char);

                std::io::stdout().flush()?;
            }
            '[' => {
                if stack[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        i += 1;
                        if i >= chars.len() {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Unmatched '['",
                            ));
                        }
                        if chars[i] == '[' {
                            depth += 1;
                        } else if chars[i] == ']' {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(i);
                }
            }
            ']' => {
                if stack[ptr] != 0 {
                    if let Some(&loop_start) = loop_stack.last() {
                        i = loop_start;
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Unmatched ']'",
                        ));
                    }
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

pub fn run(f: String) {
    let data = std::fs::read_to_string(f).unwrap_or_default();

    let chars = data.chars().collect::<Vec<char>>();

    let mut stack: Vec<u8> = vec![0; 30000];

    let mut user_stack: Vec<u8> = Vec::new();

    let mut ptr: u64 = 0;

    let mut i = 0;

    let mut loop_stack: Vec<usize> = Vec::new();

    while i < chars.len() {
        match chars[i] {
            '1' => {
                if ptr == 29999 {
                    ptr = 0;
                } else {
                    ptr += 1;
                }
            }
            '2' => {
                if ptr == 0 {
                    ptr = 29999;
                } else {
                    ptr -= 1;
                }
            }
            '3' => {
                if stack[ptr as usize] == 255 {
                    stack[ptr as usize] = 0;
                } else {
                    stack[ptr as usize] += 1;
                }
            }
            '4' => {
                if stack[ptr as usize] == 0 {
                    stack[ptr as usize] = 255;
                } else {
                    stack[ptr as usize] -= 1;
                }
            }
            '5' => {
                let mut buf: [u8; 1] = [0];

                std::io::stdin().read(&mut buf).unwrap_or_default();

                stack[ptr as usize] = *buf.first().unwrap_or(&0);
            }
            '6' => {
                print!("{}", stack[ptr as usize] as char);

                std::io::stdout().flush().unwrap_or_default();
            }
            '7' => {
                for byte in &user_stack {
                    print!("{}", *byte as char);
                }

                std::io::stdout().flush().unwrap_or(());
            }
            '8' => {
                user_stack.push(stack[ptr as usize]);
            }
            '9' => {
                user_stack.pop();
            }
            'a' => {
                user_stack.clear();
            }
            'b' => {
                stack[ptr as usize] = user_stack.len() as u8;
            }
            'c' => {
                if stack[ptr as usize] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        i += 1;
                        if i >= chars.len() {
                            return;
                        }
                        if chars[i] == 'c' {
                            depth += 1;
                        } else if chars[i] == 'd' {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(i);
                }
            }
            'd' => {
                if stack[ptr as usize] != 0 {
                    if let Some(&loop_start) = loop_stack.last() {
                        i = loop_start;
                    } else {
                        return;
                    }
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }
        i += 1;
    }
}
