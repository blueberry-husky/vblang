use crate::value::{interpret_string, Value};
use std::{borrow::Cow, collections::HashMap, fs::read_to_string, io::stdin};

use crate::lexer::{InputType, JumpTo, RandType, Token};
use array_init::array_init;
use rand::Rng;

pub fn interpret(tokens: Vec<Token>) {
    use Token::*;
    use Value::*;
    let mut line: usize = 0;
    let max = tokens.len();
    let mut labels: HashMap<&str, usize> = HashMap::new();
    let mut buffer = Uninitialized;
    let mut registers: [Value; u8::MAX as usize + 1] = array_init(|_| Uninitialized);
    tokens.iter().enumerate().for_each(|(i, x)| match x {
        Label(name) => {
            labels.insert(name, i);
            ()
        }
        _ => (),
    });
    while line < max {
        let token = unsafe { tokens.get_unchecked(line) };
        line += 1;
        // println!("{token:?}");
        match token {
            Reset => {
                buffer = Uninitialized;
                registers = array_init(|_| Uninitialized);
            }
            Dump => {
                println!("Buffer: {:?}\nRegisters: {:?}", buffer, registers)
            }
            Display(idx) => {
                if let Some(idx) = idx {
                    println!("{}", registers[*idx as usize])
                } else {
                    println!("{}", buffer)
                };
            }
            Clear(idx) => {
                if let Some(idx) = idx {
                    registers[*idx as usize] = Uninitialized;
                } else {
                    buffer = Uninitialized;
                };
            }
            Compare(a, b, jump) => {
                use JumpTo::*;
                if registers[*a as usize] == registers[*b as usize] {
                    match jump {
                        Absolute(dest) => {
                            if dest < &max {
                                line = *dest;
                            } else {
                                panic!("Attempt to jump to line out of bounds!\nline: {line},\ntoken: {token:?}")
                            }
                        }
                        Relative(dest) => line = (line as isize + *dest) as usize,
                        Labeled(label) => {
                            if let Some(dest) = labels.get(label) {
                                if dest < &max {
                                    line = *dest;
                                } else {
                                    panic!("Attempt to jump to line out of bounds!\nline: {line},\ntoken: {token:?}")
                                }
                            } else {
                                panic!("Attempt to jump to undefined label!\nline: {line}, token:\n{token:?}")
                            }
                        }
                    }
                }
            }
            CopyRegToBuffer(idx) => buffer = registers[*idx as usize].clone(),
            CopyBufferToReg(idx) => registers[*idx as usize] = buffer.clone(),
            Set(idx, value) => registers[*idx as usize] = interpret_string(Cow::Borrowed(value)),
            Shell(command) => {
                println!("Would execute {command}")
            }
            Print(text) => println!("{text}"),
            Label(_) => (),
            Jump(jump) => {
                use JumpTo::*;
                match jump {
                    Absolute(dest) => {
                        if dest < &max {
                            line = *dest;
                        } else {
                            panic!("Attempt to jump to line out of bounds!\nline: {line},\ntoken: {token:?}")
                        }
                    }
                    Relative(dest) => line = (line as isize + *dest) as usize,
                    Labeled(label) => {
                        if let Some(dest) = labels.get(label) {
                            if dest < &max {
                                line = *dest;
                            } else {
                                panic!("Attempt to jump to line out of bounds!\nline: {line},\ntoken: {token:?}")
                            }
                        } else {
                            panic!("Attempt to jump to undefined label!\nline: {line},\ntoken: {token:?}")
                        }
                    }
                }
            }
            DeleteFile(filename) => {
                if let Err(err) = std::fs::remove_file(filename) {
                    panic!("Attempt to delete file that doesn't exist, aborting!\nline: {line},\ntoken: {token:?}\nError: {err:?}")
                }
            }
            ConditionalJump(idx, jump) => {
                let value = &registers[*idx as usize];
                if value == &Integer(0) || value == &Float(0.0) {
                    use JumpTo::*;
                    match jump {
                        Absolute(dest) => {
                            if dest < &max {
                                line = *dest;
                            } else {
                                panic!("Attempt to jump to line out of bounds!\nline: {line},\ntoken: {token:?}")
                            }
                        }
                        Relative(dest) => line = (line as isize + *dest) as usize,
                        Labeled(label) => {
                            if let Some(dest) = labels.get(label) {
                                if dest < &max {
                                    line = *dest;
                                } else {
                                    panic!("Attempt to jump to line out of bounds! line:\n{line},\ntoken: {token:?}")
                                }
                            } else {
                                panic!("Attempt to jump to undefined label!\nline: {line},\ntoken: {token:?}")
                            }
                        }
                    }
                }
            }
            LoadFile(idx, filename) => {
                if let Ok(value) = read_to_string(filename) {
                    registers[*idx as usize] = interpret_string(Cow::Owned(value))
                } else {
                    panic!(
                        "Could not read file {filename}, aborting!\nline: {line},\ntoken: {token:?}"
                    )
                }
            }
            SaveFile(idx, filename) => {
                if let Err(err) = std::fs::write(filename, registers[*idx as usize].to_string()) {
                    panic!("Encountered an error when trying to write to file, aborting!\nline: {line}\ntoken: {token:?}\nerror:{err:?}")
                }
                ()
            }
            Input(idx, input_type) => {
                use InputType as it;
                let stdin = stdin();
                let mut buf = std::string::String::new();
                let out = loop {
                    buf.clear();
                    stdin
                        .read_line(&mut buf)
                        .expect("Failed to read line from STDIN");
                    buf.pop();
                    match buf.chars().last() {
                        Some('\r') => {
                            buf.pop();
                            ()
                        }
                        _ => (),
                    }

                    match input_type {
                        it::String => break String(buf),
                        it::Integer => {
                            if let Ok(n) = buf.trim().parse() {
                                break Integer(n);
                            } else {
                                println!("Does that look like an integer to you?")
                            }
                        }
                        it::Float => {
                            if let Ok(n) = buf.trim().parse() {
                                break Float(n);
                            } else {
                                println!("Does that look like a float to you?")
                            }
                        }
                    }
                };
                registers[*idx as usize] = out
            }
            Swap(idx1, idx2) => registers.swap(*idx1 as usize, *idx2 as usize),
            Add(idx1, idx2) => {
                registers[*idx1 as usize] =
                    registers[*idx1 as usize].to_owned() + registers[*idx2 as usize].to_owned()
            }
            Subtract(idx1, idx2) => {
                registers[*idx1 as usize] =
                    registers[*idx1 as usize].to_owned() - registers[*idx2 as usize].to_owned()
            }
            Multiply(idx1, idx2) => {
                registers[*idx1 as usize] =
                    registers[*idx1 as usize].to_owned() * registers[*idx2 as usize].to_owned()
            }
            Divide(idx1, idx2) => {
                registers[*idx1 as usize] =
                    registers[*idx1 as usize].to_owned() / registers[*idx2 as usize].to_owned()
            }
            Rem(idx1, idx2) => {
                registers[*idx1 as usize] =
                    registers[*idx1 as usize].to_owned() % registers[*idx2 as usize].to_owned()
            }
            Increment(idx) => {
                registers[*idx as usize] = registers[*idx as usize].to_owned() + Integer(1)
            }
            Decrement(idx) => {
                registers[*idx as usize] = registers[*idx as usize].to_owned() - Integer(1)
            }
            Rand(idx, rand_type) => {
                use RandType as rt;
                let mut rng = rand::thread_rng();
                registers[*idx as usize] = match rand_type {
                    rt::Integer(start, end) => Integer(rng.gen_range(*start..=*end)),
                    rt::Float(start, end) => Float(rng.gen_range(*start as f64..=*end as f64)),
                    rt::String(len) => {
                        let mut buf = std::string::String::with_capacity(*len);
                        while buf.len() < *len {
                            buf.push(rng.gen())
                        }
                        String(buf)
                    }
                }
            }
            n => println!("{n:?}"),
        }
    }
}
