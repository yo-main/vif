pub mod errors;
pub mod io;
pub mod time;
use vif_compiler::NativeFunction;
use vif_compiler::NativeFunctionCallee;
use vif_objects::stack::Stack;
use vif_objects::value::Value;

pub fn execute_native_call<'v>(
    stack: &Stack<'v>,
    arg_count: usize,
    func: &NativeFunction,
) -> Result<Value<'v>, errors::NativeError> {
    let stack_start = stack.len() - arg_count - 1;

    let res = match func.function {
        NativeFunctionCallee::GetTime => Value::Integer(time::get_time()?),
        NativeFunctionCallee::Print => {
            io::print(stack.get_slice(stack_start + 1))?;
            Value::None
        }
    };

    Ok(res)
}
