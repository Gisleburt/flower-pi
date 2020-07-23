// mod array;
mod clock;
mod interface;
mod value;

pub type LedMessage = [u8; 4];

// pub use array::LedArray;
pub use clock::LedClock;
pub use interface::{LedInterface, LedWritable};
pub use value::LedValue;
