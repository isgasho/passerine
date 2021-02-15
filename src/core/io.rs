use crate::common::data::Data;

/// Prints some data to stdout with a trailing newline.
pub fn println(data: Data) -> Result<Data, String> {
    println!("{}", data);
    return Ok(data);
}

/// Prints some data to stdout without a trailing newline.
pub fn print(data: Data) -> Result<Data, String> {
    print!("{}", data);
    return Ok(data);
}
