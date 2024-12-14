use crate::error::CompilerError;
use crate::value::LLVMValue;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use vif_objects::ast;

pub struct Builder<'ctx> {
    pub context: &'ctx inkwell::context::Context,
    pub builder: inkwell::builder::Builder<'ctx>,
}

impl<'ctx> Builder<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context) -> Self {
        Builder {
            context: context,
            builder: context.create_builder(),
        }
    }
    pub fn create_module(&self, module_name: &str) -> inkwell::module::Module {
        self.context.create_module(module_name)
    }

    fn get_pointer(&self, typing: &ast::Typing) -> inkwell::types::IntType<'ctx> {
        self.context.i64_type() // make this smarter lol
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

    pub fn declare_variable(
        &self,
        token: &ast::Variable,
        value: LLVMValue<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match value {
            LLVMValue::Int(i) => {
                let ptr = self
                    .builder
                    .build_alloca(i.get_type(), token.name.as_str())
                    .map_err(|e| CompilerError::LLVM(format!("{e}")))?;
                self.builder.build_store(ptr, i);
                Ok(LLVMValue::Ptr(ptr))
            }
            LLVMValue::Ptr(_) => unreachable!(),
            LLVMValue::LoadedVariable(_) => unreachable!(),
        }
    }

    fn declare_function(
        &self,
        function: &ast::Function,
        module: &Module<'ctx>,
    ) -> inkwell::values::FunctionValue<'ctx> {
        let function_ptr_type = self.get_pointer(&function.typing);

        let args = function
            .params
            .iter()
            .map(|t| self.get_pointer(&t.typing).into())
            .collect::<Vec<BasicMetadataTypeEnum>>();

        let llvm_function = function_ptr_type.fn_type(&args, false);
        module.add_function(function.name.as_str(), llvm_function, None)
    }

    pub fn declare_user_function(
        &self,
        function: &ast::Function,
        module: &Module<'ctx>,
    ) -> inkwell::basic_block::BasicBlock<'ctx> {
        let block = self
            .context
            .append_basic_block(self.declare_function(function, module), "entry");

        self.set_position_at(block);

        block
    }

    pub fn get_current_block(&self) -> Option<inkwell::basic_block::BasicBlock<'ctx>> {
        self.builder.get_insert_block()
    }

    pub fn set_position_at(&self, block: inkwell::basic_block::BasicBlock<'ctx>) {
        self.builder.position_at_end(block);
    }

    pub fn value_int(&self, i: i64) -> LLVMValue<'ctx> {
        let value_type = self.context.i64_type();
        if i >= 0 {
            LLVMValue::Int(value_type.const_int(i as u64, false))
        } else {
            LLVMValue::Int(value_type.const_int(i as u64, true))
        }
    }

    pub fn load_variable<T>(
        &self,
        name: &str,
        value: &LLVMValue<'ctx>,
        t: T,
    ) -> Result<LLVMValue<'ctx>, CompilerError>
    where
        T: inkwell::types::BasicType<'ctx>,
    {
        match value {
            LLVMValue::Ptr(ptr) => Ok(LLVMValue::LoadedVariable(
                self.builder
                    .build_load(t, *ptr, name)
                    .map_err(|e| CompilerError::LLVM(format!("{e}")))?,
            )),
            _ => unreachable!(),
        }
    }

    pub fn return_statement(&self, value: &LLVMValue<'ctx>) -> Result<(), CompilerError> {
        self.builder
            .build_return(Some(value))
            .map_err(|e| CompilerError::LLVM(format!("{}", e)))?;

        Ok(())
    }
}
