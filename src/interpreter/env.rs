// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::value::Value;

pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, FunctionDef>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub params: Vec<String>,
    pub body: Vec<crate::core::ast::Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
        env.setup_builtins();
        env
    }

    fn setup_builtins(&mut self) {
        self.variables.insert("真".to_string(), Value::Bool(true));
        self.variables.insert("假".to_string(), Value::Bool(false));

        self.variables.insert("true".to_string(), Value::Bool(true));
        self.variables.insert("false".to_string(), Value::Bool(false));
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }

    pub fn set_function(&mut self, name: String, func: FunctionDef) {
        self.functions.insert(name, func);
    }

    pub fn save_state(&self) -> EnvironmentState {
        EnvironmentState {
            variables: self.variables.clone(),
        }
    }

    pub fn load_state(&mut self, state: EnvironmentState) {
        self.variables = state.variables;
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
