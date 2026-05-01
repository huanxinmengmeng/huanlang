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

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
    List(Vec<Value>),
    Map(std::collections::HashMap<String, Value>),
    Closure(Vec<String>, Vec<crate::core::ast::Stmt>),
    Ok(Box<Value>),
    Err(Box<Value>),
}

use std::sync::mpsc::SyncSender;

#[derive(Debug)]
pub struct ChannelSender {
    sender: SyncSender<Value>,
}

impl ChannelSender {
    pub fn new(sender: SyncSender<Value>) -> Self {
        ChannelSender { sender }
    }
    
    pub fn send(&self, value: Value) -> Result<(), String> {
        self.sender.send(value).map_err(|e| e.to_string())
    }
}

#[derive(Debug)]
pub struct ChannelReceiver {
    receiver: Receiver<Value>,
}

impl ChannelReceiver {
    pub fn new(receiver: Receiver<Value>) -> Self {
        ChannelReceiver { receiver }
    }
    
    pub fn recv(&self) -> Result<Value, String> {
        self.receiver.recv().map_err(|e| e.to_string())
    }
    
    pub fn try_recv(&self) -> Result<Value, String> {
        self.receiver.try_recv().map_err(|e| e.to_string())
    }
}

pub struct ChannelData {
    sender: SyncSender<Value>,
    receiver: Receiver<Value>,
}

impl ChannelData {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
        ChannelData { sender, receiver }
    }
    
    pub fn send(&self, value: Value) -> Result<(), String> {
        self.sender.send(value).map_err(|e| e.to_string())
    }
    
    pub fn recv(&self) -> Result<Value, String> {
        self.receiver.recv().map_err(|e| e.to_string())
    }
}

pub type SharedChannelSender = Arc<ChannelSender>;
pub type SharedChannelReceiver = Arc<ChannelReceiver>;

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Char(c) => c.to_string(),
            Value::String(s) => s.clone(),
            Value::Unit => "()".to_string(),
            Value::List(items) => {
                let items: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Map(map) => {
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Closure(params, _) => {
                format!("||({})", params.join(", "))
            }
            Value::Ok(v) => format!("成功({})", v.to_string()),
            Value::Err(v) => format!("错误({})", v.to_string()),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Value::Int(_) => "整数",
            Value::Float(_) => "浮点数",
            Value::Bool(_) => "布尔",
            Value::Char(_) => "字符",
            Value::String(_) => "字符串",
            Value::Unit => "单元",
            Value::List(_) => "列表",
            Value::Map(_) => "映射",
            Value::Closure(_, _) => "闭包",
            Value::Ok(_) => "成功",
            Value::Err(_) => "错误",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Unit, Value::Unit) => true,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Closure(p1, b1), Value::Closure(p2, b2)) => p1 == p2 && b1 == b2,
            (Value::Ok(a), Value::Ok(b)) => a == b,
            (Value::Err(a), Value::Err(b)) => a == b,
            _ => false,
        }
    }
}
