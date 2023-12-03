pub mod errors;
pub mod io;
pub mod time;
use zeus_compiler::NativeFunction;
use zeus_compiler::NativeFunctionCallee;
use zeus_values::value::Value;

pub fn execute_native_call<'v>(
    stack: &Vec<Value<'v>>,
    arg_count: usize,
    func: &NativeFunction,
) -> Result<Value<'v>, errors::NativeError> {
    let stack_start = stack.len() - arg_count - 1;

    let res = match func.function {
        NativeFunctionCallee::GetTime => Value::Integer(time::get_time()?),
        NativeFunctionCallee::Print => {
            io::print(&stack[stack_start..])?;
            Value::None
        }
    };

    Ok(res)
}
