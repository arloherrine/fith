use itertools;
use std::io::{Write, Read};

pub struct Interpreter {
    data_stack: Vec<i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { data_stack: Vec::new()}
    }

    pub fn duplicate(&self) -> Interpreter {
        Interpreter { data_stack: self.data_stack.clone() }
    }

    pub fn stack_display(&self) -> String {
        itertools::join(self.data_stack.iter().cloned().map(|i| i.to_string()), " ")
    }

    pub fn execute_line(&mut self, line: &str) -> Result<String, String> {
        line.split_whitespace()
            .map(|token| self.execute_token(token))
            .fold(Ok("".to_string()), |acc, result| {
                match acc {
                    Ok(prefix) => match result {
                            Ok(s) => Ok(prefix + &s),
                            Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            })
    }

    fn execute_token(&mut self, token: &str) -> Result<String, String> {
        match token.parse::<i32>() {
            Ok(intVal) => {
                self.data_stack.push(intVal);
                Ok("".to_string())
            },
            Err(_) => {
                match token {
                    "." => if let Some(a) = self.data_stack.pop() {
                        Ok(a.to_string())
                    } else {
                        Err(format!("ERROR: Can't print, empty stack"))
                    },
                    "drop" => if let Some(a) = self.data_stack.pop() {
                        Ok("".to_string())
                    } else {
                        Err(format!("ERROR: Can't drop, empty stack"))
                    },
                    "dup" => if let Some(a) = self.data_stack.pop() {
                        self.data_stack.push(a);
                        self.data_stack.push(a);
                        Ok("".to_string())
                    } else {
                        Err(format!("ERROR: Can't dup, empty stack"))
                    },
                    "swap" => if let (Some(a), Some(b)) = (self.data_stack.pop(), self.data_stack.pop()) {
                        self.data_stack.push(a);
                        self.data_stack.push(b);
                        Ok("".to_string())
                    } else {
                        Err(format!("ERROR: Can't swap, less than 2 elements on the stack"))
                    },
                    "rot" => if let (Some(a), Some(b), Some(c)) = (self.data_stack.pop(), self.data_stack.pop(), self.data_stack.pop()) {
                        self.data_stack.push(b);
                        self.data_stack.push(a);
                        self.data_stack.push(c);
                        Ok("".to_string())
                    } else {
                        Err(format!("ERROR: Can't rot, less than 3 elements on the stack"))
                    },
                    "+" => self.execute_binary_op("add", |a, b| a + b),
                    "-" => self.execute_binary_op("subtract", |a, b| a - b),
                    "*" => self.execute_binary_op("multiply", |a, b| a * b),
                    "/" => self.execute_binary_op("divide", |a, b| a / b),
                    "mod" => self.execute_binary_op("mod", |a, b| a % b),
                    _ => {
                        Err(format!("ERROR: Unrecognized token: {}", token))
                    }
                }
            }
        }
    }

    fn execute_binary_op<F>(&mut self, name: &str, op: F) -> Result<String, String>
        where F: Fn(i32, i32) -> i32 {
        /*
        if let Some(a) = self.data_stack.pop() {
            if let Some(b) = self.data_stack.pop() {
                (a, b)
            } else {
                None
            }
        } else {
            None
        }
        */
        if let (Some(a), Some(b)) = (self.data_stack.pop(), self.data_stack.pop()) {
            self.data_stack.push(op(a, b));
            Ok("".to_string())
        } else {
            Err(format!("ERROR: Can't {}, less than two elements on the stack", name))
        }
    }
}