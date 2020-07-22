mod array;
mod value;

pub type LedMessage = [u8; 4];

pub use array::LedArray;
pub use value::LedValue;
