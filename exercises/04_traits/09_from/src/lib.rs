// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

use std::convert::From;

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32
{
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

impl From<i32> for WrappingU32
{
    fn from(new_value: i32) -> Self {
        WrappingU32 { value: new_value as u32 }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
    let int_val: i32 = 37;
    let wrapping = WrappingU32::from(int_val);
}
