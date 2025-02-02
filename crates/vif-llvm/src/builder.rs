use std::any::Any;
use std::borrow::Borrow;
use std::os::unix::process::CommandExt;

use crate::error::CompilerError;
use inkwell::basic_block::BasicBlock;
use inkwell::llvm_sys::{LLVMCallConv, LLVMValueKind};
use inkwell::module::Module;
use inkwell::types::{AnyType, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, PointerType};
use inkwell::values::{
    AnyValue, AsValueRef, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue,
    PointerValue,
};
use inkwell::AddressSpace;
use vif_objects::ast::{self, Typing};

#[derive(Clone, Debug)]
pub struct VariablePointer<'ctx> {
    ptr: PointerValue<'ctx>,
    typing: Typing,
}

impl<'ctx> VariablePointer<'ctx> {
    pub fn get_basic_value_enum(&self) -> BasicMetadataValueEnum<'ctx> {
        BasicMetadataValueEnum::PointerValue(self.ptr)
    }

    pub fn get_typing(&self) -> &Typing {
        &self.typing
    }
}

#[derive(Clone, Debug)]
pub struct FunctionPointer<'ctx> {
    ptr: FunctionValue<'ctx>,
    typing: Typing,
}

impl<'ctx> FunctionPointer<'ctx> {
    pub fn get_function_parameters(&self) -> Vec<PointerValue<'ctx>> {
        self.ptr
            .get_params()
            .iter()
            .map(|p| p.into_pointer_value())
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct RawValue<'ctx> {
    value: BasicValueEnum<'ctx>,
    typing: Typing,
}

impl<'ctx> RawValue<'ctx> {
    fn new(value: BasicValueEnum<'ctx>, typing: Typing) -> Self {
        RawValue { value, typing }
    }
}

#[derive(Clone, Debug)]
pub enum LLVMValue<'ctx> {
    RawValue(RawValue<'ctx>),
    Variable(VariablePointer<'ctx>),
    Function(FunctionPointer<'ctx>),
}

impl<'ctx> std::fmt::Display for LLVMValue<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawValue(r) => write!(f, "RawValue {}", r.value),
            Self::Variable(v) => write!(f, "Variable {}", v.ptr),
            Self::Function(func) => write!(f, "Function {}", func.ptr),
        }
    }
}

impl<'ctx> LLVMValue<'ctx> {
    pub fn new_value(value: BasicValueEnum<'ctx>, typing: Typing) -> Self {
        LLVMValue::RawValue(RawValue::new(value, typing))
    }

    pub fn new_function(function: FunctionValue<'ctx>, typing: Typing) -> Self {
        LLVMValue::Function(FunctionPointer {
            ptr: function,
            typing,
        })
    }

    pub fn new_variable(variable: PointerValue<'ctx>, typing: Typing) -> Self {
        LLVMValue::Variable(VariablePointer {
            ptr: variable,
            typing,
        })
    }

    pub fn get_variable(&self) -> VariablePointer<'ctx> {
        match self {
            Self::Variable(v) => v.clone(),
            t => unreachable!("Not a variable: {}", t),
        }
    }

    pub fn get_function_value(&self) -> &FunctionPointer<'ctx> {
        match self {
            Self::Function(f) => f,
            _ => unreachable!(),
        }
    }

    pub fn as_pointer(&self) -> PointerValue {
        match self {
            Self::RawValue(_) => unreachable!(),
            Self::Variable(v) => v.ptr,
            Self::Function(f) => f.ptr.as_global_value().as_pointer_value(),
        }
    }

    pub fn as_value(&self) -> BasicValueEnum<'ctx> {
        match self {
            Self::RawValue(v) => v.value,
            Self::Variable(v) => unreachable!(),
            Self::Function(f) => unreachable!(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::RawValue(_) => "raw value".to_owned(), // or unreacheable?
            Self::Variable(v) => v.ptr.get_name().to_str().unwrap().to_owned(),
            Self::Function(f) => f.ptr.get_name().to_str().unwrap().to_owned(),
        }
    }

    pub fn get_typing(&self) -> Typing {
        match self {
            Self::RawValue(v) => v.typing.clone(),
            Self::Variable(v) => v.typing.clone(),
            Self::Function(f) => f.typing.clone(),
        }
    }
}
pub struct Builder<'ctx> {
    pub context: &'ctx inkwell::context::Context,
    pub builder: inkwell::builder::Builder<'ctx>,
}

impl<'ctx> Builder<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context) -> Self {
        Builder {
            context,
            builder: context.create_builder(),
        }
    }
    pub fn create_module(&self, module_name: &str) -> inkwell::module::Module {
        self.context.create_module(module_name)
    }

    fn get_llvm_type(&self, typing: &ast::Typing) -> inkwell::types::BasicTypeEnum<'ctx> {
        match &typing.r#type {
            ast::Type::Int => self.context.i64_type().as_basic_type_enum(),
            ast::Type::Float => self.context.f64_type().as_basic_type_enum(),
            ast::Type::String => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
            ast::Type::Bool => self.context.bool_type().as_basic_type_enum(),
            ast::Type::None => self.context.bool_type().as_basic_type_enum(),
            ast::Type::Callable(c) => self.get_pointer(&c.output),
            ast::Type::Unknown => panic!("cannot convert unknown to llvm type"),
            ast::Type::KeyWord => panic!("cannot convert keyword to llvm type"),
        }
    }

    fn get_pointer(&self, typing: &ast::Typing) -> inkwell::types::BasicTypeEnum<'ctx> {
        match &typing.r#type {
            ast::Type::Int => self.context.i64_type().as_basic_type_enum(),
            ast::Type::Float => self.context.f64_type().as_basic_type_enum(),
            ast::Type::String => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
            ast::Type::Bool => self.context.bool_type().as_basic_type_enum(),
            ast::Type::None => self.context.bool_type().as_basic_type_enum(),
            ast::Type::Callable(c) => self.get_pointer(&c.output),
            ast::Type::Unknown => panic!("cannot convert unknown to llvm type"),
            ast::Type::KeyWord => panic!("cannot convert keyword to llvm type"),
        }
    }

    pub fn global_string(
        &self,
        name: &str,
        value: &str,
    ) -> Result<inkwell::values::GlobalValue<'ctx>, CompilerError> {
        self.builder
            .build_global_string_ptr(value, name)
            .map_err(|e| CompilerError::LLVM(format!("{}", e)))
    }

    // pub fn in_memory_string(
    //     &self,
    //     value: &str,
    // ) -> Result<inkwell::values::BasicValueEnum<'ctx>, CompilerError> {
    //     let array_type = self.context.i8_type().array_type(value.len() as u32);
    //     let array_alloca = self.builder.build_alloca(array_type, "");

    //     self.builder
    //         .build_global_string_ptr(value, name)
    //         .map_err(|e| CompilerError::LLVM(format!("{}", e)))
    // }

    pub fn declare_variable(
        &self,
        token: &ast::Variable,
        value: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let v = match value {
            LLVMValue::RawValue(v) => v.value,
            LLVMValue::Variable(v) => v.ptr.as_basic_value_enum(),
            LLVMValue::Function(f) => f
                .ptr
                .as_global_value()
                .as_pointer_value()
                .as_basic_value_enum(),
        };
        self.allocate_and_store_value(v, token.name.as_str(), token.typing.clone())
    }

    pub fn allocate_and_store_value(
        &self,
        value: BasicValueEnum<'ctx>,
        name: &str,
        typing: Typing,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let ptr = if let BasicValueEnum::PointerValue(p) = value {
            p
        } else {
            let ptr = self
                .builder
                .build_alloca(value.get_type(), name)
                .map_err(|e| CompilerError::LLVM(format!("{e}")))?;
            self.store_value(ptr, value)?;
            ptr
        };

        Ok(LLVMValue::new_variable(ptr, typing))
    }

    pub fn store_value(
        &self,
        ptr: PointerValue<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), CompilerError> {
        self.builder
            .build_store(ptr, value)
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(())
    }

    pub fn get_new_ptr(&self) -> PointerType<'ctx> {
        self.context.ptr_type(AddressSpace::default())
    }

    fn declare_function(&self, function: &ast::Function, module: &Module<'ctx>) -> LLVMValue<'ctx> {
        let function_ptr_type = self.get_new_ptr();

        let args = function
            .params
            .iter()
            .map(|_| self.context.ptr_type(AddressSpace::default()).into())
            .collect::<Vec<BasicMetadataTypeEnum>>();

        let llvm_function = function_ptr_type.fn_type(&args, false);

        LLVMValue::new_function(
            module.add_function(function.name.as_str(), llvm_function, None),
            function.typing.clone(),
        )
    }

    pub fn declare_user_function(
        &self,
        function: &ast::Function,
        module: &Module<'ctx>,
    ) -> LLVMValue<'ctx> {
        self.declare_function(function, module)
    }

    pub fn create_function_block(
        &self,
        function: &LLVMValue<'ctx>,
        block_name: &str,
    ) -> inkwell::basic_block::BasicBlock<'ctx> {
        let block = self
            .context
            .append_basic_block(function.get_function_value().ptr.clone(), block_name);

        self.set_position_at(block);

        block
    }

    pub fn create_block(&self, block_name: &str) -> inkwell::basic_block::BasicBlock<'ctx> {
        let function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        self.context.append_basic_block(function, block_name)
    }

    pub fn goto_block(&self, block: BasicBlock) -> Result<(), CompilerError> {
        self.builder
            .build_unconditional_branch(block)
            .map_err(|_| CompilerError::LLVM("Cannot go to block".to_owned()))?;

        Ok(())
    }

    pub fn create_branche(
        &self,
        expression: LLVMValue<'ctx>,
        then_block: BasicBlock<'ctx>,
        else_block: BasicBlock<'ctx>,
    ) -> Result<(), CompilerError> {
        let value = match &expression {
            LLVMValue::Variable(v) => self.load_variable("", v)?,
            LLVMValue::RawValue(_) => expression.as_value(),
            LLVMValue::Function(_) => unreachable!(),
        };

        self.builder
            .build_conditional_branch(
                value.as_basic_value_enum().into_int_value(),
                then_block,
                else_block,
            )
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(())
    }

    pub fn get_current_block(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.builder.get_insert_block()
    }

    pub fn set_position_at(&self, block: inkwell::basic_block::BasicBlock<'ctx>) {
        self.builder.position_at_end(block);
    }

    pub fn value_int(&self, i: i64) -> BasicValueEnum<'ctx> {
        let value_type = self.context.i64_type();
        if i >= 0 {
            BasicValueEnum::IntValue(value_type.const_int(i as u64, false))
        } else {
            BasicValueEnum::IntValue(value_type.const_int(i as u64, true))
        }
    }

    pub fn value_bool(&self, i: bool) -> BasicValueEnum<'ctx> {
        let value_type = self.context.bool_type();
        if i {
            BasicValueEnum::IntValue(value_type.const_int(i as u64, false))
        } else {
            BasicValueEnum::IntValue(value_type.const_int(i as u64, true))
        }
    }

    pub fn value_float(&self, i: f64) -> BasicValueEnum<'ctx> {
        let value_type = self.context.f64_type();
        if i >= 0.0 {
            BasicValueEnum::FloatValue(value_type.const_float(i as f64))
        } else {
            BasicValueEnum::FloatValue(value_type.const_float(i as f64))
        }
    }

    pub fn load_llvm_value(
        &self,
        name: &str,
        value: &LLVMValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match value {
            LLVMValue::RawValue(r) => Ok(r.value.clone()),
            LLVMValue::Variable(var) => self.load_variable(name, var),
            v => unreachable!("Not a variable: {v}"),
        }
    }

    fn load_variable(
        &self,
        name: &str,
        var: &VariablePointer<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CompilerError> {
        match var.get_typing().r#type.get_concrete_type() {
            ast::Type::String => Ok(var.ptr.as_basic_value_enum()),
            _ => self
                .builder
                .build_load(self.get_llvm_type(&var.typing), var.ptr, name)
                .map_err(|e| CompilerError::LLVM(format!("{e}"))),
        }
    }

    pub fn return_statement(&self, value: &LLVMValue<'ctx>) -> Result<(), CompilerError> {
        match value {
            LLVMValue::RawValue(v) => {
                let var = self.allocate_and_store_value(v.value, "", v.typing.clone())?;

                self.builder
                    .build_return(Some(&var.get_variable().ptr))
                    .map_err(|e| CompilerError::LLVM(format!("{e}")))
            }
            LLVMValue::Variable(v) => self
                .builder
                .build_return(Some(&v.ptr))
                .map_err(|e| CompilerError::LLVM(format!("{e}"))),
            LLVMValue::Function(f) => self
                .builder
                .build_return(Some(&f.ptr.as_global_value().as_pointer_value()))
                .map_err(|e| CompilerError::LLVM(format!("{e}"))),
        }?;

        Ok(())
    }

    pub fn call(
        &self,
        function: &FunctionPointer<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let call_result = self
            .builder
            .build_direct_call(function.ptr.clone(), args, name)
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        if let Some(v) = call_result.try_as_basic_value().left() {
            Ok(self.allocate_and_store_value(v, "", function.typing.clone())?)
        } else {
            self.allocate_and_store_value(
                BasicValueEnum::IntValue(self.context.i64_type().const_int(0, false)),
                "",
                Typing::new(true, ast::Type::None),
            )
        }
    }

    pub fn add(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = self.load_llvm_value("", &value_left)?;
        let r = self.load_llvm_value("", &value_right)?;

        let result = self
            .builder
            .build_int_add(l.into_int_value(), r.into_int_value(), "coucou")
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(LLVMValue::new_value(
            result.as_basic_value_enum(),
            value_left.get_typing(),
        ))
    }

    pub fn sub(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = self.load_llvm_value("", &value_left)?;
        let r = self.load_llvm_value("", &value_right)?;

        let result = self
            .builder
            .build_int_sub(l.into_int_value(), r.into_int_value(), "coucou")
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(LLVMValue::new_value(
            result.as_basic_value_enum(),
            value_left.get_typing(),
        ))
    }

    pub fn divide(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = self.load_llvm_value("", &value_left)?;
        let r = self.load_llvm_value("", &value_right)?;

        let result = self
            .builder
            .build_int_signed_div(l.into_int_value(), r.into_int_value(), "coucou")
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(LLVMValue::new_value(
            result.as_basic_value_enum(),
            value_left.get_typing(),
        ))
    }

    pub fn multiply(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = self.load_llvm_value("", &value_left)?;
        let r = self.load_llvm_value("", &value_right)?;

        let result = self
            .builder
            .build_int_mul(l.into_int_value(), r.into_int_value(), "coucou")
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;

        Ok(LLVMValue::new_value(
            result.as_basic_value_enum(),
            value_left.get_typing(),
        ))
    }
}
