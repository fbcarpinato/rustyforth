use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum StackValue {
    Number(f64),
    Boolean(bool),
    String(String),
}

#[derive(Debug)]
pub enum InterpreterError {
    StackUnderflow,
    TypeMismatch(&'static str),
    DivisionByZero,
    UnknownWord(String),
}

type WordFn = fn(&mut Interpreter) -> Result<(), InterpreterError>;

pub struct Interpreter {
    stack: Vec<StackValue>,
    words: HashMap<String, WordFn>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            stack: Vec::new(),
            words: HashMap::new(),
        };

        interp.register_builtins();
        interp
    }

    fn register_builtins(&mut self) {
        self.words.insert("print".into(), |interp| {
            let v = interp.pop()?;
            match v {
                StackValue::Number(n) => println!("{}", n),
                StackValue::Boolean(b) => println!("{}", b),
                StackValue::String(s) => println!("{}", s),
            }
            Ok(())
        });

        self.words.insert("dup".into(), |interp| {
            let v = interp.peek()?.clone();
            interp.stack.push(v);
            Ok(())
        });

        self.words.insert("drop".into(), |interp| {
            interp.pop()?;
            Ok(())
        });

        self.words.insert("swap".into(), |interp| {
            if interp.stack.len() < 2 {
                return Err(InterpreterError::StackUnderflow);
            }
            let b = interp.stack.pop().unwrap();
            let a = interp.stack.pop().unwrap();
            interp.stack.push(b);
            interp.stack.push(a);
            Ok(())
        });

        self.words
            .insert("+".into(), |interp| interp.binary_op(|a, b| Ok(a + b)));
        self.words
            .insert("-".into(), |interp| interp.binary_op(|a, b| Ok(a - b)));
        self.words
            .insert("*".into(), |interp| interp.binary_op(|a, b| Ok(a * b)));
        self.words.insert("/".into(), |interp| {
            interp.binary_op(|a, b| {
                if b == 0.0 {
                    Err(InterpreterError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            })
        });
    }

    fn pop(&mut self) -> Result<StackValue, InterpreterError> {
        self.stack.pop().ok_or(InterpreterError::StackUnderflow)
    }

    fn peek(&self) -> Result<&StackValue, InterpreterError> {
        self.stack.last().ok_or(InterpreterError::StackUnderflow)
    }

    fn pop_number(&mut self) -> Result<f64, InterpreterError> {
        match self.pop()? {
            StackValue::Number(n) => Ok(n),
            _ => Err(InterpreterError::TypeMismatch("number")),
        }
    }

    fn binary_op(
        &mut self,
        op: fn(f64, f64) -> Result<f64, InterpreterError>,
    ) -> Result<(), InterpreterError> {
        let b = self.pop_number()?;
        let a = self.pop_number()?;
        let r = op(a, b)?;
        self.stack.push(StackValue::Number(r));
        Ok(())
    }

    pub fn interpret(&mut self, tokens: Vec<Token>) -> Result<(), InterpreterError> {
        for token in tokens {
            match token {
                Token::NumberLiteral(v) => self.stack.push(StackValue::Number(v)),
                Token::StringLiteral(s) => self.stack.push(StackValue::String(s)),
                Token::BooleanLiteral(b) => self.stack.push(StackValue::Boolean(b)),

                Token::Identifier(name) => {
                    if let Some(word) = self.words.get(&name) {
                        word(self)?;
                    } else {
                        return Err(InterpreterError::UnknownWord(name));
                    }
                }

                Token::Plus => (self.words["+"])(self)?,
                Token::Minus => (self.words["-"])(self)?,
                Token::Asterisk => (self.words["*"])(self)?,
                Token::Slash => (self.words["/"])(self)?,

                Token::EOF => break,
            }
        }
        Ok(())
    }
}
