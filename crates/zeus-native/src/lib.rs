pub mod errors;
pub mod time;
use zeus_compiler::NativeFunction;
use zeus_compiler::NativeFunctionCallee;
use zeus_values::value::Value;

pub fn execute_native_call(
    stack: &mut Vec<Value>,
    arg_count: usize,
    func: &NativeFunction,
) -> Result<(), errors::NativeError> {
    let stack_start = stack.len() - arg_count - 1;

    let res = match func.function {
        NativeFunctionCallee::GetTime => Value::Integer(time::get_time(stack)?),
    };

    stack.push(res);

    Ok(())
}
