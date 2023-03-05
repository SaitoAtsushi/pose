use std::result::Result;

pub struct Pose<T: std::iter::Iterator<Item = char>> {
    src: std::iter::Peekable<T>,
}

#[derive(Debug, PartialEq)]
pub enum PoseType {
    String(String),
    Number(f64),
    Symbol(String),
    List(Vec<PoseType>),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum PoseError {
    InvalidString,
    InvalidNumber,
    NothingClosingParenthesis,
}

type PoseResult = Result<PoseType, PoseError>;

impl<T: Iterator<Item = char>> Pose<T> {
    fn is_signsym_2nd(ch: &char) -> bool {
        ch.is_alphabetic() || "!$&*+-/<=>_.?@".contains(*ch)
    }

    fn is_signsym_cont(ch: &char) -> bool {
        Self::is_signsym_2nd(ch) || ch.is_ascii_digit()
    }

    pub fn new(src: T) -> Self {
        Self {
            src: src.peekable(),
        }
    }

    fn skip_space(&mut self) {
        while self
            .src
            .next_if(|&ch| ch.is_ascii_whitespace() || ch == '\x0b')
            .is_some()
        {}
    }

    fn read_string(&mut self) -> Option<String> {
        let mut str = String::new();

        loop {
            match self.src.next()? {
                '"' => break,
                '\\' => match self.src.next()? {
                    '\\' => str.push('\\'),
                    '"' => str.push('"'),
                    _ => None?,
                },
                ch => str.push(ch),
            }
        }
        Some(str)
    }

    fn read_integer_part(&mut self) -> Option<f64> {
        let ch = self.src.next_if(char::is_ascii_digit)?;
        if ch == '0' {
            return Some(0.0);
        }
        let mut num: f64 = ch.to_digit(10).unwrap() as f64;
        while let Some(ch) = self.src.next_if(char::is_ascii_digit) {
            num *= 10.0;
            num += ch.to_digit(10).unwrap() as f64;
        }
        Some(num)
    }

    fn read_decimal_part(&mut self) -> Option<f64> {
        Some(if self.src.next_if_eq(&'.').is_some() {
            let mut mask: f64 = 0.1;
            let mut num: f64 = mask
                * self
                    .src
                    .next_if(char::is_ascii_digit)?
                    .to_digit(10)
                    .unwrap() as f64;
            while let Some(ch) = self.src.next_if(char::is_ascii_digit) {
                mask *= 0.1;
                num += mask * (ch.to_digit(10)? as f64);
            }
            num
        } else {
            0.0
        })
    }

    fn read_exp_part(&mut self) -> Option<i32> {
        if self.src.next_if(|&ch| ch == 'e').is_some() {
            let flag = self.src.next_if(|&ch| ch == '+' || ch == '-')?;
            let mut num = self
                .src
                .next_if(char::is_ascii_digit)?
                .to_digit(10)
                .unwrap() as i32;
            while let Some(ch) = self.src.next_if(char::is_ascii_digit) {
                num *= 10;
                num += ch.to_digit(10).unwrap() as i32;
            }
            if flag == '-' {
                num *= -1;
            }
            Some(num)
        } else {
            Some(0)
        }
    }

    fn read_number(&mut self) -> Option<f64> {
        let num = self.read_integer_part()?;
        let num = num + self.read_decimal_part()?;
        let e = self.read_exp_part()?;
        Some(num * 10.0_f64.powi(e))
    }

    pub fn read(&mut self) -> PoseResult {
        self.skip_space();
        match self.src.peek() {
            None => {
                self.src.next();
                Ok(PoseType::EOF)
            }
            Some(';') => {
                self.src.next();
                while let Some(item) = self.src.next() {
                    if item == '\r' || item == '\n' {
                        break;
                    }
                }
                self.read()
            }
            Some('(') => {
                self.src.next();
                let mut v = Vec::<PoseType>::new();
                while {
                    self.skip_space();
                    !self.src.next_if_eq(&')').is_some()
                } {
                    let item = self.read()?;
                    if item == PoseType::EOF {
                        Err(PoseError::NothingClosingParenthesis)?
                    }
                    v.push(item);
                }
                Ok(PoseType::List(v))
            }
            Some('"') => {
                self.src.next();
                Ok(PoseType::String(
                    self.read_string().ok_or(PoseError::InvalidString)?,
                ))
            }
            Some('-') => {
                self.src.next();
                if self.src.peek().map_or(false, char::is_ascii_digit) {
                    Ok(PoseType::Number(
                        -self.read_number().ok_or(PoseError::InvalidNumber)?,
                    ))
                } else if let Some(ch) = self.src.next_if(Self::is_signsym_2nd) {
                    let mut sym = String::from('-');
                    sym.push(ch);
                    while let Some(ch) = self.src.next_if(Self::is_signsym_cont) {
                        sym.push(ch);
                    }
                    Ok(PoseType::Symbol(sym))
                } else {
                    Ok(PoseType::Symbol(String::from("-")))
                }
            }
            Some('+') => {
                self.src.next();
                if let Some(ch) = self.src.next_if(Self::is_signsym_2nd) {
                    let mut sym = String::from('+');
                    sym.push(ch);
                    while let Some(ch) = self.src.next_if(Self::is_signsym_cont) {
                        sym.push(ch);
                    }
                    Ok(PoseType::Symbol(sym))
                } else {
                    Ok(PoseType::Symbol(String::from("+")))
                }
            }
            Some(ch) if ch.is_ascii_digit() => Ok(PoseType::Number(
                self.read_number().ok_or(PoseError::InvalidNumber)?,
            )),
            Some(ch) if ch.is_ascii_alphabetic() || "!$&*+-/<=>_".contains(*ch) => {
                let mut name = String::from(self.src.next().unwrap());
                while let Some(ch) = self
                    .src
                    .next_if(|&ch| ch.is_ascii_alphanumeric() || "!$&*+-/<=>_.?@".contains(ch))
                {
                    name.push(ch)
                }
                Ok(PoseType::Symbol(name))
            }
            _ => Ok(PoseType::Number(1.0)),
        }
    }
}
