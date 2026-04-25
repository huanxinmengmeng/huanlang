// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// AST到MLIR的转换
// 实现规范第10.3节的完整转换过程

use std::collections::HashMap;
use crate::core::ast::{Program, Item, Expr, Stmt, Function, BinaryOp, UnaryOp};
use crate::core::mlir::ops::*;
use crate::core::mlir::types::*;
use crate::core::lexer::token::SourceSpan;

/// 转换错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// 类型错误
    TypeError(String),
    /// 表达式错误
    ExprError(String),
    /// 语句错误
    StmtError(String),
    /// 函数错误
    FunctionError(String),
    /// 其他错误
    Other(String),
}

/// AST到MLIR转换器
pub struct AstToMlirConverter {
    /// 变量环境（变量名到操作的映射）
    variables: HashMap<String, Box<dyn HuanOp>>,
    /// 作用域栈（用于嵌套作用域管理）
    scope_stack: Vec<HashMap<String, Box<dyn HuanOp>>>,
}

impl AstToMlirConverter {
    /// 创建新的转换器
    pub fn new() -> Self {
        AstToMlirConverter {
            variables: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }
    
    /// 进入新作用域
    pub fn enter_scope(&mut self) {
        let old_vars = std::mem::take(&mut self.variables);
        self.scope_stack.push(old_vars);
    }
    
    /// 离开当前作用域
    pub fn exit_scope(&mut self) {
        if let Some(old_vars) = self.scope_stack.pop() {
            self.variables = old_vars;
        }
    }
    
    /// 定义变量
    pub fn define_variable(&mut self, name: String, value: Box<dyn HuanOp>) {
        self.variables.insert(name, value);
    }
    
    /// 查找变量
    pub fn lookup_variable(&self, name: &str) -> Option<&Box<dyn HuanOp>> {
        self.variables.get(name)
    }
    
    /// 转换程序
    pub fn convert_program(&mut self, program: &Program) -> Result<Vec<Box<dyn HuanOp>>, ConversionError> {
        let mut ops = Vec::new();
        
        for item in program {
            if let Some(op) = self.convert_item(item)? {
                ops.push(op);
            }
        }
        
        Ok(ops)
    }
    
    /// 转换项目
    fn convert_item(&mut self, item: &Item) -> Result<Option<Box<dyn HuanOp>>, ConversionError> {
        match item {
            Item::Function(func) => Ok(Some(self.convert_function(func)?)),
            Item::Struct(_s) => Err(ConversionError::Other("结构体转换".to_string())),
            Item::Trait(_t) => Err(ConversionError::Other("特性转换".to_string())),
            Item::Impl(_i) => Err(ConversionError::Other("实现转换".to_string())),
            Item::Module(_m) => Err(ConversionError::Other("模块转换".to_string())),
            Item::Import(_import) => {
                // 导入语句在解析阶段已处理，跳过
                Ok(None)
            }
            Item::Extern(_ext) => Err(ConversionError::Other("外部块转换".to_string())),
            Item::TypeAlias(_alias) => Err(ConversionError::Other("类型别名转换".to_string())),
            Item::Global(_global) => Err(ConversionError::Other("全局变量转换".to_string())),
        }
    }
    
    /// 转换函数
    fn convert_function(&mut self, func: &Function) -> Result<Box<dyn HuanOp>, ConversionError> {
        self.enter_scope();
        
        let params: Vec<_> = func.params
            .iter()
            .map(|(name, ty)| {
                let huan_ty = from_ast_type(ty)
                    .map_err(|e| ConversionError::TypeError(e))?;
                Ok((name.name.clone(), huan_ty))
            })
            .collect::<Result<_, ConversionError>>()?;
        
        let return_type = from_ast_type(&func.return_type)
            .map_err(|e| ConversionError::TypeError(e))?;
        
        let mut body = Vec::new();
        for stmt in &func.body {
            let op = self.convert_stmt(stmt)?;
            body.push(op);
        }
        
        self.exit_scope();
        
        Ok(Box::new(HanshuOp {
            name: func.name.name.clone(),
            params,
            return_type,
            body,
            span: func.span.clone(),
        }))
    }
    
    /// 转换语句
    fn convert_stmt(&mut self, stmt: &Stmt) -> Result<Box<dyn HuanOp>, ConversionError> {
        match stmt {
            Stmt::Let { name, ty, value, span } => {
                let var_name = name.name.clone();
                let var_type = if let Some(ty) = ty {
                    from_ast_type(ty)
                        .map_err(|e| ConversionError::TypeError(e))?
                } else {
                    return Err(ConversionError::Other("处理无类型的变量".to_string()));
                };
                let value = self.convert_expr(value)?;
                
                let ling_op = Box::new(LingOp {
                    sym_name: var_name.clone(),
                    var_type,
                    value: value.clone(),
                    span: span.clone(),
                });
                
                self.define_variable(var_name, value);
                Ok(ling_op)
            }
            Stmt::Const { name, ty, value, span } => {
                let const_name = name.name.clone();
                let const_type = if let Some(ty) = ty {
                    from_ast_type(ty)
                        .map_err(|e| ConversionError::TypeError(e))?
                } else {
                    return Err(ConversionError::Other("处理无类型的常量".to_string()));
                };
                let value = self.convert_expr(value)?;
                
                let ding_op = Box::new(DingOp {
                    sym_name: const_name.clone(),
                    const_type,
                    value: value.clone(),
                    span: span.clone(),
                });
                
                self.define_variable(const_name, value);
                Ok(ding_op)
            }
            Stmt::Assign { target, value, span } => {
                let target = self.convert_expr(target)?;
                let value = self.convert_expr(value)?;
                Ok(Box::new(SheweiOp {
                    target,
                    value,
                    span: span.clone(),
                }))
            }
            Stmt::If { cond, then_block, else_ifs: _, else_block, span } => {
                let condition = self.convert_expr(cond)?;
                
                let mut then_block_ops = Vec::new();
                self.enter_scope();
                for s in then_block {
                    let op = self.convert_stmt(s)?;
                    then_block_ops.push(op);
                }
                self.exit_scope();
                
                let else_block_ops = if let Some(eb) = else_block {
                    self.enter_scope();
                    let mut eb_ops = Vec::new();
                    for s in eb {
                        let op = self.convert_stmt(s)?;
                        eb_ops.push(op);
                    }
                    self.exit_scope();
                    Some(eb_ops)
                } else {
                    None
                };
                
                Ok(Box::new(RuoOp {
                    condition,
                    then_block: then_block_ops,
                    else_block: else_block_ops,
                    span: span.clone(),
                }))
            }
            Stmt::While { cond, body, span } => {
                let condition = self.convert_expr(cond)?;
                
                let mut body_ops = Vec::new();
                self.enter_scope();
                for s in body {
                    let op = self.convert_stmt(s)?;
                    body_ops.push(op);
                }
                self.exit_scope();
                
                Ok(Box::new(DangOp {
                    condition,
                    body: body_ops,
                    span: span.clone(),
                }))
            }
            Stmt::Return(ret, span) => {
                let value = if let Some(expr) = ret {
                    Some(self.convert_expr(expr)?)
                } else {
                    None
                };
                
                Ok(Box::new(FanhuiOp {
                    value,
                    span: span.clone(),
                }))
            }
            Stmt::Expr(e, _) => self.convert_expr(e),
            _ => todo!("其他语句转换"),
        }
    }
    
    /// 转换表达式
    fn convert_expr(&mut self, expr: &Expr) -> Result<Box<dyn HuanOp>, ConversionError> {
        match expr {
            Expr::IntLit(i, _) => Ok(Box::new(IntLitOp {
                value: *i,
                span: SourceSpan::default(),
            })),
            Expr::FloatLit(f, _) => Ok(Box::new(FloatLitOp {
                value: *f,
                span: SourceSpan::default(),
            })),
            Expr::StringLit(s, _) => Ok(Box::new(StringLitOp {
                value: s.clone(),
                span: SourceSpan::default(),
            })),
            Expr::CharLit(c, _) => Ok(Box::new(CharLitOp {
                value: *c,
                span: SourceSpan::default(),
            })),
            Expr::BoolLit(b, _) => Ok(Box::new(BoolLitOp {
                value: *b,
                span: SourceSpan::default(),
            })),
            Expr::Null(_) => Ok(Box::new(NullOp {
                span: SourceSpan::default(),
            })),
            Expr::Ident(ident) => {
                Ok(Box::new(IdentOp {
                    name: ident.name.clone(),
                    span: ident.span.clone(),
                }))
            }
            Expr::BinaryOp { op, left, right, span } => {
                match op {
                    BinaryOp::Add => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(JiaOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Sub => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(JianOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Mul => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(ChengOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Div => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(ChuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Mod => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(QuyuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Gt => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(DayuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Lt => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(XiaoyuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Ge => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(DayuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Le => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(XiaoyuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Eq => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(DengyuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Ne => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(DengyuOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::And => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(QieOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    BinaryOp::Or => {
                        let lhs = self.convert_expr(left)?;
                        let rhs = self.convert_expr(right)?;
                        Ok(Box::new(HuoOp {
                            lhs,
                            rhs,
                            span: span.clone(),
                        }))
                    }
                    _ => Err(ConversionError::ExprError(format!("Unsupported binary op {:?}", op))),
                }
            }
            Expr::UnaryOp { op, expr, span } => {
                match op {
                    UnaryOp::Not => {
                        let operand = self.convert_expr(expr)?;
                        Ok(Box::new(FeiOp {
                            operand,
                            span: span.clone(),
                        }))
                    }
                    UnaryOp::Neg => {
                        let zero = Box::new(IntLitOp {
                            value: 0,
                            span: span.clone(),
                        });
                        let operand = self.convert_expr(expr)?;
                        Ok(Box::new(JianOp {
                            lhs: zero,
                            rhs: operand,
                            span: span.clone(),
                        }))
                    }
                    UnaryOp::BitNot => {
                        let operand = self.convert_expr(expr)?;
                        Ok(Box::new(FeiOp {
                            operand,
                            span: span.clone(),
                        }))
                    }
                }
            }
            Expr::Call { func, args, span } => {
                let mut converted_args = Vec::new();
                for arg in args {
                    converted_args.push(self.convert_expr(arg)?);
                }
                let callee = match **func {
                    Expr::Ident(ref ident) => ident.name.clone(),
                    _ => return Err(ConversionError::ExprError("Function call must have an identifier as the function name".to_string())),
                };
                Ok(Box::new(DiaoyongOp {
                    callee,
                    args: converted_args,
                    span: span.clone(),
                }))
            }
            Expr::List(list, span) => {
                let mut elements = Vec::new();
                for e in list {
                    elements.push(self.convert_expr(e)?);
                }
                Ok(Box::new(LiebiaoOp {
                    elements,
                    span: span.clone(),
                }))
            }
            Expr::Index { target, index, span } => {
                let container = self.convert_expr(target)?;
                let index = self.convert_expr(index)?;
                Ok(Box::new(SuoyinOp {
                    container,
                    index,
                    span: span.clone(),
                }))
            }
            Expr::Field { target, field, span } => {
                let obj = self.convert_expr(target)?;
                Ok(Box::new(ZiduanOp {
                    object: obj,
                    field: field.name.clone(),
                    span: span.clone(),
                }))
            }
            Expr::Map(pairs, span) => {
                let mut keys = Vec::new();
                let mut values = Vec::new();
                for (k, v) in pairs {
                    keys.push(self.convert_expr(k)?);
                    values.push(self.convert_expr(v)?);
                }
                Ok(Box::new(ZidianOp {
                    keys,
                    values,
                    span: span.clone(),
                }))
            }
            Expr::IfExpr { cond, then_expr, else_expr, span } => {
                let condition = self.convert_expr(cond)?;
                let then_block_ops = vec![self.convert_expr(then_expr)?];
                let else_block_ops = vec![self.convert_expr(else_expr)?];
                Ok(Box::new(RuoOp {
                    condition,
                    then_block: then_block_ops,
                    else_block: Some(else_block_ops),
                    span: span.clone(),
                }))
            }
            Expr::Match { expr, arms, default, span } => {
                let value = self.convert_expr(expr)?;
                let mut mlir_arms = Vec::new();
                for (pattern, arm_expr) in arms {
                    let _pattern = pattern;
                    let arm_ops = vec![self.convert_expr(arm_expr)?];
                    mlir_arms.push((Box::new(IntLitOp { value: 0, span: SourceSpan::default() }) as Box<dyn HuanOp>, arm_ops));
                }
                let default_arm = if let Some(def_expr) = default {
                    Some(vec![self.convert_expr(def_expr)?])
                } else {
                    None
                };
                Ok(Box::new(PipeiOp {
                    value,
                    arms: mlir_arms,
                    default_arm,
                    span: span.clone(),
                }))
            }
            _ => Err(ConversionError::ExprError("Unsupported expression type".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::*;
    use crate::core::lexer::token::SourceSpan;

    #[test]
    fn test_converter_basic() {
        let _converter = AstToMlirConverter::new();
    }
}
