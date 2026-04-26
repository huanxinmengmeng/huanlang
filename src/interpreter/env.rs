// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
