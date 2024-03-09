pub mod errors;
pub mod io;
pub mod time;
use vif_objects::function::NativeFunction;
use vif_objects::function::NativeFunctionCallee;
use vif_objects::stack::Stack;
use vif_objects::stack_value::StackValue;

pub fn execute_native_call<'v>(
    stack: &Stack<'v>,
    arg_count: usize,
    func: &NativeFunction,
) -> Result<StackValue<'v>, errors::NativeError> {
    let stack_start = stack.len() - arg_count;

    let res = match func.function {
        NativeFunctionCallee::GetTime => StackValue::Integer(time::get_time()?),
        NativeFunctionCallee::Print => {
            io::print(stack.get_slice(stack_start))?;
            StackValue::None
        }
        NativeFunctionCallee::Sleep => {
            time::sleep(stack.peek_last())?;
            StackValue::None
        }
    };

    Ok(res)
}
