mod arena;
pub use arena::*;

pub use by_address::ByAddress as NodeAsKey;

mod character_reader;
pub use character_reader::*;

mod shared_array;
pub use shared_array::*;

pub fn default<T: Default>() -> T {
    T::default()
}