use crate::errors::NativeError;
use vif_objects::value::Value;

pub fn print(iter: Vec<&Value<'_>>) -> Result<(), NativeError> {
    println!(
        "{}",
        iter.iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(())
}
