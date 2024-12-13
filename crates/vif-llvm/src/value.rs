use inkwell;

#[derive(Debug)]
pub enum LLVMValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
}

unsafe impl<'ctx> inkwell::values::AsValueRef for LLVMValue<'ctx> {
    fn as_value_ref(&self) -> inkwell::llvm_sys::prelude::LLVMValueRef {
        match self {
            Self::Int(i) => i.as_value_ref(),
        }
    }
}

unsafe impl<'ctx> inkwell::values::AnyValue<'ctx> for LLVMValue<'ctx> {
    fn as_any_value_enum(&self) -> inkwell::values::AnyValueEnum<'ctx> {
        match self {
            Self::Int(i) => i.as_any_value_enum(),
        }
    }

    fn print_to_string(&self) -> inkwell::support::LLVMString {
        match self {
            Self::Int(i) => i.print_to_string(),
        }
    }

    fn is_poison(&self) -> bool {
        match self {
            Self::Int(i) => i.is_poison(),
        }
    }
}

unsafe impl<'ctx> inkwell::values::BasicValue<'ctx> for LLVMValue<'ctx> {
    fn as_basic_value_enum(&self) -> inkwell::values::BasicValueEnum<'ctx> {
        match self {
            Self::Int(i) => i.as_basic_value_enum(),
        }
    }

    fn as_instruction_value(&self) -> Option<inkwell::values::InstructionValue<'ctx>> {
        match self {
            Self::Int(i) => i.as_instruction_value(),
        }
    }

    fn get_first_use(&self) -> Option<inkwell::values::BasicValueUse> {
        match self {
            Self::Int(i) => i.get_first_use(),
        }
    }

    fn set_name(&self, name: &str) {
        match self {
            Self::Int(i) => i.set_name(name),
        }
    }
}
