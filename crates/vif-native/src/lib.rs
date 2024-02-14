pub mod errors;
pub mod io;
pub mod time;
use vif_compiler::NativeFunction;
use vif_compiler::NativeFunctionCallee;
use vif_objects::stack::Stack;
use vif_objects::stack_value::StackValue;

pub fn execute_native_call<'v>(
    stack: &Stack<'v>,
    arg_count: usize,
    func: &NativeFunction,
) -> Result<StackValue<'v>, errors::NativeError> {
    let stack_start = stack.len() - arg_count - 1;

    let res = match func.function {
        NativeFunctionCallee::GetTime => StackValue::Integer(time::get_time()?),
        NativeFunctionCallee::Print => {
            io::print(stack.get_slice(stack_start + 1))?;
            StackValue::None
        }
    };

    Ok(res)
}
