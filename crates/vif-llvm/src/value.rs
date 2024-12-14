use inkwell::{self, execution_engine::UnsafeFunctionPointer, values::AsValueRef};

#[derive(Debug, Clone)]
pub enum LLVMValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
    Ptr(inkwell::values::PointerValue<'ctx>),
    // Instruction(inkwell::values::InstructionValue<'ctx>),
    // Block(inkwell::basic_block::BasicBlock<'ctx>),
    LoadedVariable(inkwell::values::BasicValueEnum<'ctx>),
    // None,
}

unsafe impl<'ctx> inkwell::values::AsValueRef for LLVMValue<'ctx> {
    fn as_value_ref(&self) -> inkwell::llvm_sys::prelude::LLVMValueRef {
        match self {
            Self::Int(i) => i.as_value_ref(),
            Self::Ptr(p) => p.as_value_ref(),
            // Self::Instruction(i) => i.as_value_ref(),
            Self::LoadedVariable(v) => v.as_value_ref(),
        }
    }
}

unsafe impl<'ctx> inkwell::values::AnyValue<'ctx> for LLVMValue<'ctx> {
    fn as_any_value_enum(&self) -> inkwell::values::AnyValueEnum<'ctx> {
        match self {
            Self::Int(i) => i.as_any_value_enum(),
            Self::Ptr(p) => p.as_any_value_enum(),
            // Self::Instruction(i) => i.as_any_value_enum(),
            Self::LoadedVariable(v) => v.as_any_value_enum(),
        }
    }

    fn print_to_string(&self) -> inkwell::support::LLVMString {
        match self {
            Self::Int(i) => i.print_to_string(),
            Self::Ptr(p) => p.print_to_string(),
            // Self::Instruction(i) => i.print_to_string(),
            Self::LoadedVariable(v) => v.print_to_string(),
        }
    }

    fn is_poison(&self) -> bool {
        match self {
            Self::Int(i) => i.is_poison(),
            Self::Ptr(p) => p.is_poison(),
            // Self::Instruction(i) => i.is_poison(),
            Self::LoadedVariable(v) => v.is_poison(),
        }
    }
}

unsafe impl<'ctx> inkwell::values::BasicValue<'ctx> for LLVMValue<'ctx> {
    fn as_basic_value_enum(&self) -> inkwell::values::BasicValueEnum<'ctx> {
        match self {
            Self::Int(i) => i.as_basic_value_enum(),
            Self::Ptr(p) => p.as_basic_value_enum(),
            // Self::Instruction(i) => unreachable!(),
            Self::LoadedVariable(v) => v.as_basic_value_enum(),
        }
    }

    fn as_instruction_value(&self) -> Option<inkwell::values::InstructionValue<'ctx>> {
        match self {
            Self::Int(i) => i.as_instruction_value(),
            Self::Ptr(p) => p.as_instruction_value(),
            // Self::Instruction(i) => unreachable!(),
            Self::LoadedVariable(v) => v.as_instruction_value(),
        }
    }

    fn get_first_use(&self) -> Option<inkwell::values::BasicValueUse> {
        match self {
            Self::Int(i) => i.get_first_use(),
            Self::Ptr(p) => p.get_first_use(),
            // Self::Instruction(i) => i.get_first_use(),
            Self::LoadedVariable(v) => v.get_first_use(),
        }
    }

    fn set_name(&self, name: &str) {
        match self {
            Self::Int(i) => i.set_name(name),
            Self::Ptr(p) => p.set_name(name),
            // Self::Instruction(i) => unreachable!(),
            Self::LoadedVariable(v) => v.set_name(name),
        }
    }
}
