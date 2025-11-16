use crate::lexer::Token;

enum StackValue {
    Number(f64),
    Boolean(bool),
    String(String),
}

pub struct Interpreter {
    stack: Vec<StackValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { stack: Vec::new() }
    }

    pub fn interpret(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            match token {
                Token::NumberLiteral(value) => {
                    self.stack.push(StackValue::Number(value));
                }
                Token::StringLiteral(value) => {
                    self.stack.push(StackValue::String(value));
                }
                Token::BooleanLiteral(value) => {
                    self.stack.push(StackValue::Boolean(value));
                }
                Token::Plus => {
                    if let (Some(StackValue::Number(b)), Some(StackValue::Number(a))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        self.stack.push(StackValue::Number(a + b));
                    } else {
                        eprintln!("Error: '+' operator requires two numbers on the stack");
                    }
                }
                Token::Minus => {
                    if let (Some(StackValue::Number(b)), Some(StackValue::Number(a))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        self.stack.push(StackValue::Number(a - b));
                    } else {
                        eprintln!("Error: '-' operator requires two numbers on the stack");
                    }
                }
                Token::Asterisk => {
                    if let (Some(StackValue::Number(b)), Some(StackValue::Number(a))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        self.stack.push(StackValue::Number(a * b));
                    } else {
                        eprintln!("Error: '*' operator requires two numbers on the stack");
                    }
                }
                Token::Slash => {
                    if let (Some(StackValue::Number(b)), Some(StackValue::Number(a))) =
                        (self.stack.pop(), self.stack.pop())
                    {
                        if b != 0.0 {
                            self.stack.push(StackValue::Number(a / b));
                        } else {
                            eprintln!("Error: Division by zero");
                            self.stack.push(StackValue::Number(a));
                            self.stack.push(StackValue::Number(b));
                        }
                    } else {
                        eprintln!("Error: '/' operator requires two numbers on the stack");
                    }
                }
                Token::Identifier(name) => match name.as_str() {
                    "print" => {
                        if let Some(value) = self.stack.pop() {
                            match value {
                                StackValue::Number(n) => println!("{}", n),
                                StackValue::Boolean(b) => println!("{}", b),
                                StackValue::String(s) => println!("{}", s),
                            }
                        } else {
                            eprintln!("Error: 'print' requires a value on the stack");
                        }
                    }
                    "dup" => {
                        if let Some(value) = self.stack.last() {
                            match value {
                                StackValue::Number(n) => self.stack.push(StackValue::Number(*n)),
                                StackValue::Boolean(b) => self.stack.push(StackValue::Boolean(*b)),
                                StackValue::String(s) => {
                                    self.stack.push(StackValue::String(s.clone()))
                                }
                            }
                        } else {
                            eprintln!("Error: 'dup' requires a value on the stack");
                        }
                    }
                    _ => {
                        eprintln!("Error: Unknown identifier '{}'", name);
                    }
                },
                Token::EOF => break,
            }
        }
    }
}
