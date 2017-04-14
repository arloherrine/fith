use itertools;

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

    pub fn execute_line(&mut self, line: &str) -> String {
        itertools::join(line.split_whitespace().filter_map(|token| self.execute_token(token)), "")
    }

    fn execute_token(&mut self, token: &str) -> Option<String> {
        match token.parse::<i32>() {
            Ok(intVal) => {
                self.data_stack.push(intVal);
                None
            },
            Err(_) => {
                match token {
                    "." => if let Some(a) = self.data_stack.pop() {
                        Some(a.to_string())
                    } else {
                        Some("ERROR: Can't print, empty stack".to_string())
                    },
                    "drop" => if let Some(a) = self.data_stack.pop() {
                        None
                    } else {
                        Some("ERROR: Can't drop, empty stack".to_string())
                    },
                    "dup" => if let Some(a) = self.data_stack.pop() {
                        self.data_stack.push(a);
                        self.data_stack.push(a);
                        None
                    } else {
                        Some("ERROR: Can't dup, empty stack".to_string())
                    },
                    "swap" => if let (Some(a), Some(b)) = (self.data_stack.pop(), self.data_stack.pop()) {
                        self.data_stack.push(a);
                        self.data_stack.push(b);
                        None
                    } else {
                        Some("ERROR: Can't swap, less than 2 elements on the stack".to_string())
                    },
                    "rot" => if let (Some(a), Some(b), Some(c)) = (self.data_stack.pop(), self.data_stack.pop(), self.data_stack.pop()) {
                        self.data_stack.push(b);
                        self.data_stack.push(a);
                        self.data_stack.push(c);
                        None
                    } else {
                        Some("ERROR: Can't rot, less than 3 elements on the stack".to_string())
                    },
                    "+" => self.execute_binary_op("add", |a, b| a + b),
                    "-" => self.execute_binary_op("subtract", |a, b| a - b),
                    "*" => self.execute_binary_op("multiply", |a, b| a * b),
                    "/" => self.execute_binary_op("divide", |a, b| a / b),
                    "mod" => self.execute_binary_op("mod", |a, b| a % b),
                    _ => {
                        Some(format!("ERROR: Unrecognized token: {}", token))
                    }
                }
            }
        }
    }

    fn execute_binary_op<F>(&mut self, name: &str, op: F) -> Option<String>
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
            None
        } else {
            Some(format!("ERROR: Can't {}, less than two elements on the stack", name))
        }
    }
}