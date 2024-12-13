use inkwell::{self, module::Module, types::BasicMetadataTypeEnum};
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
        self.context.i32_type() // make this smarter lol
    }

    pub fn declare_function(
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

    pub fn set_cursor_to_function(&self, function: inkwell::values::FunctionValue) {
        let block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(block);
    }
}
