use std::{
    str::FromStr,
    collections::HashMap,
};

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, Clone, PartialEq)]
enum Word {
    Number(Value),
    Operator(Operator),
    Dup,
    Drop,
    Swap,
    Over,
    Command(String),
}

impl Word {
    fn parse_string<'a>(input: &mut impl Iterator<Item=&'a str>, commands: &HashMap<String, Vec<Word>>) -> Result<Vec<Word>, Error> {
        input.try_fold(vec![], |mut words, word| {
            if commands.contains_key(word) {
                words.push(Word::Command(word.to_string()));
            } else {
                words.push((&word).parse::<Word>()?);
            }
            Ok(words)
        })
    }
}

impl FromStr for Word {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        match s {
            "dup" => Ok(Word::Dup),
            "drop" => Ok(Word::Drop),
            "swap" => Ok(Word::Swap),
            "over" => Ok(Word::Over),
            _ => {
                if let Ok(v) = s.parse::<i32>() {
                    Ok(Word::Number(v))
                } else if let Ok(o) = s.parse::<Operator>() {
                    Ok(Word::Operator(o))
                } else {
                    Err(Error::UnknownWord)
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(Error::UnknownWord),
        }
    }
}

pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<String, Vec<Word>>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            commands: HashMap::default(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    fn operate(&mut self, operator: &Operator) -> Result<Value, Error> {
        let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        match operator {
            Operator::Plus => Ok(a + b),
            Operator::Minus => Ok(a - b),
            Operator::Mul => Ok(a * b),
            Operator::Div => {
                if b == 0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
        }
    }

    fn execute(&mut self, words: &[Word]) -> ForthResult {
        for word in words {
            match word {
                Word::Number(v) => self.stack.push(*v),
                Word::Operator(o) => {
                    let res = self.operate(o)?;
                    self.stack.push(res);
                },
                Word::Drop => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                },
                Word::Dup => {
                    if self.stack.len() < 1 {
                        return Err(Error::StackUnderflow);
                    }
                    self.stack.push(self.stack[self.stack.len() - 1])
                },
                Word::Swap => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(a);
                    self.stack.push(b);
                },
                Word::Over => {
                    let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(b);
                    self.stack.push(a);
                    self.stack.push(b);
                },
                Word::Command(com) => {
                    let ws = match self.commands.remove(com) {
                        Some(ws) => ws,
                        _ => return Err(Error::InvalidWord),

                    };
                    self.execute(&ws)?;
                    self.commands.insert(com.to_string(), ws);
                },
            }
        }
        Ok(())
    }

    fn handle_new_command(&mut self, input: &str) -> ForthResult {
        if input.ends_with(" ;") {
            let mut inputs = input.split(" ").collect::<Vec<&str>>();
            inputs.remove(0);
            inputs.pop();
            let name = inputs.remove(0);
            if name.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }
            let words = Word::parse_string(&mut inputs.into_iter(), &self.commands)?
                    .into_iter()
                    .fold(vec![], |mut acc, w| {
                        match w {
                            Word::Command(c) => {
                                acc.append(&mut self.commands
                                           .get(&c)
                                           .expect("Word parsing should fail if command does not exsist").clone());
                            },
                            _ => acc.push(w)
                        }
                        acc
                    });
            self.commands.insert(name.to_string(), words);
            Ok(())
        } else {
            Err(Error::InvalidWord)
        }
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        if input.len() == 0 {
            Ok(())
        } else {
            let input = input.to_lowercase();
            if input.starts_with(":") {
                self.handle_new_command(&input)?;
            } else {
                let words = Word::parse_string(&mut input.split(" "), &self.commands)?;
                self.execute(&words)?;
            }

            Ok(())
        }
    }
}
