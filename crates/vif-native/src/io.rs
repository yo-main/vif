use crate::errors::NativeError;
use vif_objects::stack_value::StackValue;

pub fn print(iter: Vec<&StackValue<'_>>) -> Result<(), NativeError> {
    println!(
        "{}",
        iter.iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(())
}
