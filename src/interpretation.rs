use std::collections::HashMap;
use crate::parserization::{Instruction};

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String)
}

impl Value {
    pub fn to_int(&self) -> i64 {
        match self {
            Value::Integer(value) => value.clone(),
            Value::String(value) => value.parse::<i64>().unwrap(),
            Value::Float(value) => value.clone() as i64,
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            Value::Integer(value) => value.clone() as f64,
            Value::String(value) => value.parse::<f64>().unwrap(),
            Value::Float(value) => value.clone(),
        }
    }

    pub fn to_string(&self) -> String{
        match self {
            Value::Integer(value) => value.to_string(),
            Value::String(value) => value.clone(),
            Value::Float(value) => value.to_string(),
        }
    }
}

pub struct Machine {
    instructions: Vec<Instruction>,

    pub stack_int: Vec<i64>,
    pub stack_float: Vec<f64>,
    pub stack_str: Vec<String>,
    pub stack: Vec<Value>,
  //  pub variables: HashMap<String, Value>
}

impl Machine {
    pub fn new(instrs: Vec<Instruction>) -> Machine {
        Machine {
            instructions: instrs,
            stack_int: Vec::<i64>::new(),
            stack_float: Vec::<f64>::new(),
            stack_str: Vec::<String>::new(),
            stack: Vec::<Value>::new(),
          //  variables: HashMap::<String, Value>::new()
        }
    }

    pub fn execute(&mut self) {
        let mut index = 0;
        loop {
            let instruction: &Instruction;
            if index < self.instructions.len() {
                instruction = self.instructions.get(index).unwrap();
                index += 1;
            } else {
                break;
            }

            match &instruction {
                &Instruction::PushInt(value) => {
                    self.stack.push(Value::Integer(value.clone()));
                }
                &Instruction::PushFloat(value) => {
                    self.stack.push(Value::Float(value.clone()));
                }
                &Instruction::PushStr(value) => {
                    self.stack.push(Value::String(value.clone()));
                }

                &Instruction::Plus => {
                    let left = self.stack.pop().unwrap();
                   // let right = self.stack.pop().unwrap();
                    match left {
                        Value::String(left_str) => {
                            let right = self.stack.pop().unwrap();
                            self.stack.push(Value::String(left_str + &right.to_string()));
                        }
                        Value::Integer(left_int) => {
                            let right = self.stack.pop().unwrap();
                                            self.stack.push(Value::Integer(left_int + right.to_int()));
                        }
                        Value::Float(left_float) => {
                            let right = self.stack.pop().unwrap();
                            self.stack.push(Value::Float(left_float + right.to_float()));
                        }
                    }
                   // self.stack.push(Value::Integer(left.to_int() + right.to_int()));
                }

                _ => break
            }
        }
    }
}