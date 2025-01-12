use crate::compiler::LLVMValue;
use crate::error::CompilerError;
use inkwell::llvm_sys::LLVMCallConv;
use inkwell::module::Module;
use inkwell::types::{AnyType, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, PointerType};
use inkwell::values::{
    AsValueRef, BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue,
};
use inkwell::AddressSpace;
use vif_objects::ast;

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

    fn get_pointer(&self, typing: &ast::Typing) -> inkwell::types::BasicTypeEnum<'ctx> {
        match &typing.r#type {
            ast::Type::Int => self.context.i64_type().as_basic_type_enum(),
            ast::Type::Float => self.context.f64_type().as_basic_type_enum(),
            ast::Type::String => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
            ast::Type::Bool => self.context.i64_type().as_basic_type_enum(),
            ast::Type::None => self.context.i64_type().as_basic_type_enum(),
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
    ) -> Result<PointerValue<'ctx>, CompilerError> {
        match value {
            LLVMValue::BasicValueEnum(v) => self.allocate_and_store_value(v, token.name.as_str()),
            LLVMValue::FunctionValue(_) => unimplemented!(),
        }
    }

    pub fn allocate_and_store_value(
        &self,
        value: BasicValueEnum<'ctx>,
        name: &str,
    ) -> Result<PointerValue<'ctx>, CompilerError> {
        let ptr = self
            .builder
            .build_alloca(value.get_type(), name)
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?;
        self.store_value(ptr, value)?;
        Ok(ptr)
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

    fn declare_function(
        &self,
        function: &ast::Function,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        let function_ptr_type = self.get_new_ptr();

        let args = function
            .params
            .iter()
            .map(|t| self.context.ptr_type(AddressSpace::default()).into())
            .collect::<Vec<BasicMetadataTypeEnum>>();

        let llvm_function = function_ptr_type.fn_type(&args, false);
        module.add_function(function.name.as_str(), llvm_function, None)
    }

    pub fn declare_user_function(
        &self,
        function: &ast::Function,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        self.declare_function(function, module)
    }

    pub fn create_function_block(
        &self,
        function: FunctionValue<'ctx>,
        block_name: &str,
    ) -> inkwell::basic_block::BasicBlock<'ctx> {
        let block = self.context.append_basic_block(function, block_name);

        self.set_position_at(block);

        block
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

    pub fn value_float(&self, i: f64) -> BasicValueEnum<'ctx> {
        let value_type = self.context.f64_type();
        if i >= 0.0 {
            BasicValueEnum::FloatValue(value_type.const_float(i as f64))
        } else {
            BasicValueEnum::FloatValue(value_type.const_float(i as f64))
        }
    }

    pub fn load_variable<T>(
        &self,
        name: &str,
        value: &BasicValueEnum<'ctx>,
        t: T,
    ) -> Result<BasicValueEnum<'ctx>, CompilerError>
    where
        T: inkwell::types::BasicType<'ctx>,
    {
        match value {
            BasicValueEnum::PointerValue(ptr) => self
                .builder
                .build_load(t, *ptr, name)
                .map_err(|e| CompilerError::LLVM(format!("{e}"))),
            v => Ok(v.clone()),
        }
    }

    pub fn return_statement(&self, value: &PointerValue<'ctx>) -> Result<(), CompilerError> {
        self.builder
            .build_return(Some(value))
            .map_err(|e| CompilerError::LLVM(format!("{}", e)))?;

        Ok(())
    }

    pub fn call(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<Option<BasicValueEnum<'ctx>>, CompilerError> {
        let call_value = self
            .builder
            .build_direct_call(function, args, name)
            .map_err(|e| CompilerError::LLVM(format!("{e}")))?
            .try_as_basic_value();

        Ok(call_value.left())
    }

    pub fn add(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = match value_left.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => return Err(CompilerError::Unknown(format!("Cannot add {:?}", v))),
        };

        let r = match value_right.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => {
                return Err(CompilerError::Unknown(format!(
                    "Cannot add {:?} with {:?}",
                    v, l
                )))
            }
        };

        Ok(LLVMValue::BasicValueEnum(BasicValueEnum::IntValue(
            self.builder
                .build_int_add(l, r, "coucou")
                .map_err(|e| CompilerError::LLVM(format!("{e}")))?,
        )))
    }

    pub fn sub(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = match value_left.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => return Err(CompilerError::Unknown(format!("Cannot add {:?}", v))),
        };

        let r = match value_right.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => {
                return Err(CompilerError::Unknown(format!(
                    "Cannot add {:?} with {:?}",
                    v, l
                )))
            }
        };

        Ok(LLVMValue::BasicValueEnum(BasicValueEnum::IntValue(
            self.builder
                .build_int_sub(l, r, "coucou")
                .map_err(|e| CompilerError::LLVM(format!("{e}")))?,
        )))
    }

    pub fn divide(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = match value_left.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => return Err(CompilerError::Unknown(format!("Cannot add {:?}", v))),
        };

        let r = match value_right.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => {
                return Err(CompilerError::Unknown(format!(
                    "Cannot add {:?} with {:?}",
                    v, l
                )))
            }
        };

        Ok(LLVMValue::BasicValueEnum(BasicValueEnum::IntValue(
            self.builder
                .build_int_signed_div(l, r, "coucou")
                .map_err(|e| CompilerError::LLVM(format!("{e}")))?,
        )))
    }

    pub fn multiply(
        &self,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let l = match value_left.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => return Err(CompilerError::Unknown(format!("Cannot add {:?}", v))),
        };

        let r = match value_right.get_basic_value_enum() {
            BasicValueEnum::IntValue(i) => i,
            v => {
                return Err(CompilerError::Unknown(format!(
                    "Cannot add {:?} with {:?}",
                    v, l
                )))
            }
        };

        Ok(LLVMValue::BasicValueEnum(BasicValueEnum::IntValue(
            self.builder
                .build_int_mul(l, r, "coucou")
                .map_err(|e| CompilerError::LLVM(format!("{e}")))?,
        )))
    }
}
