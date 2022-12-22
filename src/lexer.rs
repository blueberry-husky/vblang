// pub enum Meth {
//     Divide()
// }

use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq)]
pub enum JumpTo<'a> {
    Absolute(usize),
    Relative(isize),
    Labeled(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputType {
    String,
    Integer,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum RandType {
    String(usize),
    Integer(isize, isize),
    Float(f64, f64),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Reset,
    Dump,
    Clear(Option<u8>),
    Display(Option<u8>),
    Set(u8, &'a str),
    Shell(&'a str),
    Print(&'a str),
    CopyRegToBuffer(u8),
    CopyBufferToReg(u8),
    Compare(u8, u8, JumpTo<'a>),
    Jump(JumpTo<'a>),
    Label(&'a str),
    DeleteFile(&'a str),
    LoadFile(u8, &'a str),
    SaveFile(u8, &'a str),
    Input(u8, InputType),
    ConditionalJump(u8, JumpTo<'a>),
    Swap(u8, u8),
    Add(u8, u8),
    Subtract(u8, u8),
    Multiply(u8, u8),
    Divide(u8, u8),
    Rem(u8, u8),
    Increment(u8),
    Decrement(u8),
    Rand(u8, RandType),
    Unknown(u32, &'a str, Option<&'a str>),
}

pub fn learn_to_read<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = input
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let (op, data) = match x.split_once(' ') {
                Some((first, "")) => (first, None),
                Some((first, rest)) => (first, Some(rest)),
                None => (x, None), // If there was no space in the line
            };
            lex_i_guess(i as u32, op, data)
        })
        .collect();
    tokens.retain(|x| match x {
        Token::Unknown(..) => false,
        _ => true,
    });
    tokens
}

fn lex_i_guess<'a>(line: u32, op: &'a str, data: Option<&'a str>) -> Token<'a> {
    use Token::*;
    match op.as_bytes() {
        b"rst" => Reset,
        b"dmp" => Dump,
        b"clr" => Clear(data.map(|x| x.as_bytes().get(0).unwrap_or(&65)).copied()),
        b"dsp" => Display(data.map(|x| x.as_bytes().get(0).unwrap_or(&65)).copied()),
        &[b'r', b'g', id] => {
            use RandType as rt;
            let Some(mut args) = data.map(|x| x.splitn(3, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg), Some(arg1), arg2) = (args.next(),args.next(), args.next()) else {
                return Unknown(line, op, data)
            };

            match id {
                b's' => {
                    let Ok(len) = arg1.parse() else {
                        return Unknown(line, op, data)
                    };
                    Rand(*reg.as_bytes().get(0).unwrap_or(&65), rt::String(len))
                }
                b'i' => {
                    let (Ok(start), Ok(end)) = (arg1.parse(), arg2.unwrap_or("a").parse()) else {
                        return Unknown(line, op, data)
                    };
                    Rand(
                        *reg.as_bytes().get(0).unwrap_or(&65),
                        rt::Integer(start, end),
                    )
                }
                b'f' => {
                    let (Ok(start), Ok(end)) = (arg1.parse(), arg2.unwrap_or("a").parse()) else {
                        return Unknown(line, op, data)
                    };
                    Rand(*reg.as_bytes().get(0).unwrap_or(&65), rt::Float(start, end))
                }
                _ => panic!(
                    "Incorrect `rg` syntax! line: {line}\n{op} {}",
                    data.unwrap_or("")
                ),
            }
        }
        &[b'i', b's', id] => {
            use JumpTo::*;
            ConditionalJump(
                id,
                match data {
                    None => return Unknown(line, op, data),
                    Some(arg) if arg.starts_with(['+', '-']) => {
                        // Relative line jumps
                        let Ok(offset) = isize::from_str_radix(arg, 10) else {
                        return Unknown(line, op, data)
                };
                        if offset == 0 {
                            return Unknown(line, op, data);
                        }
                        Relative(offset)
                    }
                    Some(arg) => {
                        if let Ok(absolute) = usize::from_str_radix(arg, 10) {
                            if absolute == 0 {
                                return Unknown(line, op, data);
                            }
                            Absolute(absolute - 1)
                        } else {
                            Labeled(&arg[0..arg.bytes().take_while(|&ch| ch != b' ').count()])
                        }
                    }
                },
            )
        }
        &[b's', b't', id] => {
            let Some(data) = data else {
                return Unknown(line,op, data)
            };
            Set(id, data)
        }
        &[b'c', b'b', id] => CopyRegToBuffer(id),
        &[b'c', b'r', id] => CopyBufferToReg(id),
        &[b's', b'i', id] => Input(id, InputType::String),
        &[b'i', b'i', id] => Input(id, InputType::Integer),
        &[b'f', b'i', id] => Input(id, InputType::Float),
        b"shl" => {
            let Some(data) = data else {
                return Unknown(line, op, data)
            };
            Shell(data)
        }
        b"prt" => Print(data.unwrap_or_default()),
        b"cmp" => {
            let Some(mut args) = data.map(|x| x.splitn(3, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2), Some(jumpto)) = (args.next(), args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            use JumpTo::*;
            Compare(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
                match jumpto {
                    arg if arg.starts_with(['+', '-']) => {
                        // Relative line jumps
                        let Ok(offset) = isize::from_str_radix(arg, 10) else {
                        return Unknown(line, op, data)
                };
                        if offset == 0 {
                            return Unknown(line, op, data);
                        }
                        Relative(offset)
                    }
                    arg => {
                        if let Ok(absolute) = usize::from_str_radix(arg, 10) {
                            if absolute == 0 {
                                return Unknown(line, op, data);
                            }
                            Absolute(absolute - 1)
                        } else {
                            Labeled(&arg[0..arg.bytes().take_while(|&ch| ch != b' ').count()])
                        }
                    }
                },
            )
        }
        b"swp" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Swap(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"add" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Add(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"sub" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Subtract(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"mul" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Multiply(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"div" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Divide(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"rem" => {
            let Some(mut args) = data.map(|x| x.splitn(2, ' ')) else {
                return Unknown(line, op, data)
            };
            let (Some(reg1), Some(reg2)) = (args.next(), args.next()) else {
                return Unknown(line, op, data)
            };
            Rem(
                *reg1.as_bytes().get(0).unwrap_or(&65),
                *reg2.as_bytes().get(0).unwrap_or(&65),
            )
        }
        b"inc" => {
            let Some(arg) = data else {
                return Unknown(line, op, data)
            };
            Increment(*arg.as_bytes().get(0).unwrap_or(&65))
        }
        b"dec" => {
            let Some(arg) = data else {
                return Unknown(line, op, data)
            };
            Decrement(*arg.as_bytes().get(0).unwrap_or(&65))
        }
        b"jmp" => {
            use JumpTo::*;
            match data {
                None => Unknown(line, op, data),
                Some(arg) if arg.starts_with(['+', '-']) => {
                    // Relative line jumps
                    let Ok(offset) = isize::from_str_radix(arg, 10) else {
                        return Unknown(line, op, data)
                };
                    if offset == 0 {
                        return Unknown(line, op, data);
                    }
                    Jump(Relative(offset))
                }
                Some(arg) => {
                    let Ok(absolute) = usize::from_str_radix(arg, 10) else {
                        return Jump(Labeled(&arg[0..arg.bytes().take_while(|&ch| ch != b' ').count()]))
                    };
                    if absolute == 0 {
                        return Unknown(line, op, data);
                    }
                    Jump(Absolute(absolute - 1))
                }
            }
        }
        b"lbl" => {
            let Some(data) = data else {
                return Unknown(line, op, data)
            };
            Label(&data[0..data.bytes().take_while(|ch| ch != &b' ').count()])
        }
        b"dlf" => {
            let Some(data) = data else {
                return Unknown(line, op, data)
            };
            DeleteFile(data)
        }
        &[b'l', b'd', id] => {
            let Some(data) = data else {
                return Unknown(line, op, data)
            };
            LoadFile(id, data)
        }
        &[b's', b'r', id] => {
            let Some(data) = data else {
                return Unknown(line, op, data)
            };
            SaveFile(id, data)
        }
        _ => Unknown(line, op, data),
    }
}
