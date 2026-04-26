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

cfg_if::cfg_if! {
    if #[cfg(feature = "llvm")] {
        use inkwell::context::Context;
        use inkwell::module::Module;
        use inkwell::builder::Builder;
        use inkwell::execution_engine::ExecutionEngine;
        use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
        use std::collections::HashMap;
        use crate::core::ast::*;

        pub struct LlvmBackend<'ctx> {
            context: &'ctx Context,
            module: Module<'ctx>,
            builder: Builder<'ctx>,
            variables: HashMap<String, PointerValue<'ctx>>,
            current_function: Option<FunctionValue<'ctx>>,
        }

        impl<'ctx> LlvmBackend<'ctx> {
            pub fn new(context: &'ctx Context, name: &str) -> Self {
                let module = context.create_module(name);
                let builder = context.create_builder();

                Self {
                    context,
                    module,
                    builder,
                    variables: HashMap::new(),
                    current_function: None,
                }
            }

            pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
                for item in program {
                    self.compile_item(item)?;
                }
                Ok(())
            }

            fn compile_item(&mut self, item: &Item) -> Result<(), String> {
                match item {
                    Item::Function(func) => self.compile_function(func),
                    _ => Ok(()),
                }
            }

            fn compile_function(&mut self, func: &Function) -> Result<(), String> {
                let func_name = func.name.name.as_str();
                let return_type = self.convert_type(&func.return_type);
                let param_types: Vec<_> = func.params.iter().map(|(_, ty)| self.convert_type(ty)).collect();

                let function_type = self.context.ptr_type(inkwell::AddressSpace::Generic).fn_type(&param_types, false);
                let function = self.module.add_function(func_name, function_type, None);

                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);
                self.current_function = Some(function);

                for (i, (param, _)) in func.params.iter().enumerate() {
                    let param_value = function.get_param(i).unwrap();
                    let alloca = self.builder.build_alloca(self.convert_type(&func.params[i].1), param.name.name.as_str());
                    self.builder.build_store(alloca, param_value);
                    self.variables.insert(param.name.name.clone(), alloca);
                }

                for stmt in &func.body {
                    self.compile_stmt(stmt)?;
                }

                if !matches!(func.return_type, Type::Unit) {
                    let zero = self.context.i32_type().const_int(0, false);
                    self.builder.build_return(Some(&zero.as_basic_value_enum()));
                } else {
                    self.builder.build_return(None);
                }

                self.current_function = None;
                self.variables.clear();
                Ok(())
            }

            fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
                match stmt {
                    Stmt::Let { name, ty, value, .. } => {
                        let value = self.compile_expr(value)?;
                        let var_type = if let Some(ty) = ty {
                            self.convert_type(ty)
                        } else {
                            value.get_type()
                        };
                        let alloca = self.builder.build_alloca(var_type, name.name.as_str());
                        self.builder.build_store(alloca, value);
                        self.variables.insert(name.name.clone(), alloca);
                        Ok(())
                    }
                    Stmt::Return(expr, ..) => {
                        if let Some(expr) = expr {
                            let value = self.compile_expr(expr)?;
                            self.builder.build_return(Some(&value));
                        } else {
                            self.builder.build_return(None);
                        }
                        Ok(())
                    }
                    Stmt::Expr(expr, ..) => {
                        self.compile_expr(expr)?;
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }

            fn compile_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, String> {
                match expr {
                    Expr::IntLit(n, _) => {
                        Ok(self.context.i64_type().const_int(*n as u64, false).as_basic_value_enum())
                    }
                    Expr::FloatLit(f, _) => {
                        Ok(self.context.f64_type().const_float(*f).as_basic_value_enum())
                    }
                    Expr::StringLit(s, _) => {
                        let string_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic);
                        let global_string = self.module.add_global_string(s, "string");
                        global_string.set_linkage(inkwell::module::Linkage::Private);
                        Ok(global_string.as_pointer_value().as_basic_value_enum())
                    }
                    Expr::Ident(ident) => {
                        if let Some(var) = self.variables.get(&ident.name) {
                            Ok(self.builder.build_load(*var, ident.name.as_str()))
                        } else {
                            Err(format!("未找到变量: {}", ident.name))
                        }
                    }
                    Expr::BinaryOp { op, left, right, .. } => {
                        let left = self.compile_expr(left)?;
                        let right = self.compile_expr(right)?;
                        self.compile_binary_op(op, left, right)
                    }
                    Expr::Call { func, args, .. } => {
                        let func_name = match func {
                            Expr::Ident(ident) => ident.name.as_str(),
                            _ => return Err("仅支持直接函数调用".to_string()),
                        };

                        let evaluated_args: Result<Vec<BasicValueEnum<'ctx>>, String> =
                            args.iter().map(|arg| self.compile_expr(arg)).collect();
                        let evaluated_args = evaluated_args?;

                        let func = self.module.get_function(func_name)
                            .ok_or_else(|| format!("未找到函数: {}", func_name))?;

                        Ok(self.builder.build_call(func, &evaluated_args, "call").try_as_basic_value().unwrap_or_else(|_| {
                            self.context.void_type().as_basic_value_enum()
                        }))
                    }
                    _ => Err("未支持的表达式类型".to_string()),
                }
            }

            fn compile_binary_op(&mut self, op: &BinaryOp, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
                match op {
                    BinaryOp::Add => Ok(self.builder.build_add(left, right, "add")),
                    BinaryOp::Sub => Ok(self.builder.build_sub(left, right, "sub")),
                    BinaryOp::Mul => Ok(self.builder.build_mul(left, right, "mul")),
                    BinaryOp::Div => Ok(self.builder.build_signed_div(left, right, "div")),
                    BinaryOp::Eq => Ok(self.builder.build_int_compare(inkwell::IntPredicate::EQ, left, right, "eq")),
                    BinaryOp::Ne => Ok(self.builder.build_int_compare(inkwell::IntPredicate::NE, left, right, "ne")),
                    BinaryOp::Lt => Ok(self.builder.build_int_compare(inkwell::IntPredicate::SLT, left, right, "lt")),
                    BinaryOp::Gt => Ok(self.builder.build_int_compare(inkwell::IntPredicate::SGT, left, right, "gt")),
                    BinaryOp::Le => Ok(self.builder.build_int_compare(inkwell::IntPredicate::SLE, left, right, "le")),
                    BinaryOp::Ge => Ok(self.builder.build_int_compare(inkwell::IntPredicate::SGE, left, right, "ge")),
                    _ => Err("未支持的二元运算符".to_string()),
                }
            }

            fn convert_type(&self, ty: &Type) -> inkwell::types::BasicTypeEnum<'ctx> {
                match ty {
                    Type::Int => self.context.i64_type().as_basic_type_enum(),
                    Type::I8 => self.context.i8_type().as_basic_type_enum(),
                    Type::I16 => self.context.i16_type().as_basic_type_enum(),
                    Type::I32 => self.context.i32_type().as_basic_type_enum(),
                    Type::I64 => self.context.i64_type().as_basic_type_enum(),
                    Type::U8 => self.context.i8_type().as_basic_type_enum(),
                    Type::U16 => self.context.i16_type().as_basic_type_enum(),
                    Type::U32 => self.context.i32_type().as_basic_type_enum(),
                    Type::U64 => self.context.i64_type().as_basic_type_enum(),
                    Type::F32 => self.context.f32_type().as_basic_type_enum(),
                    Type::F64 => self.context.f64_type().as_basic_type_enum(),
                    Type::Bool => self.context.bool_type().as_basic_type_enum(),
                    Type::Char => self.context.i8_type().as_basic_type_enum(),
                    Type::String => self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic).as_basic_type_enum(),
                    Type::Unit => self.context.void_type().as_basic_type_enum(),
                    _ => self.context.i64_type().as_basic_type_enum(),
                }
            }

            pub fn write_bitcode(&self, path: &str) -> Result<(), String> {
                self.module.write_bitcode_to_path(path)
                    .map_err(|e| format!("写入位码失败: {:?}", e))
            }

            pub fn write_object_file(&self, path: &str) -> Result<(), String> {
                let execution_engine = ExecutionEngine::try_from(&self.module)
                    .map_err(|e| format!("创建执行引擎失败: {:?}", e))?;

                execution_engine.write_object_file(path)
                    .map_err(|e| format!("写入目标文件失败: {:?}", e))
            }
        }
    } else {
        use std::collections::HashMap;
        use crate::core::ast::*;

        pub struct LlvmBackend;

        impl LlvmBackend {
            pub fn new() -> Self {
                Self
            }

            pub fn compile_program(&mut self, _program: &Program) -> Result<(), String> {
                Err("LLVM 功能未启用，请使用 --features llvm 编译".to_string())
            }

            pub fn write_bitcode(&self, _path: &str) -> Result<(), String> {
                Err("LLVM 功能未启用，请使用 --features llvm 编译".to_string())
            }

            pub fn write_object_file(&self, _path: &str) -> Result<(), String> {
                Err("LLVM 功能未启用，请使用 --features llvm 编译".to_string())
            }
        }

        impl Default for LlvmBackend {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}