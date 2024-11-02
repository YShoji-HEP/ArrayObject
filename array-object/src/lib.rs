//! # ArrayObject
//! A simple self-describing array of integers, real numbers, complex numbers and strings, designed for object storage, database and single file.
//!
//! Examples
//! --------
//! Encoding and decording:
//! ```
//! use array_object::*;
//!
//! fn main() {
//!     // Convert data into binary
//!     let original = vec![1u32, 2, 3, 4];
//!     let obj: ArrayObject = original.clone().try_into().unwrap();
//!     let packed = obj.pack(); // This converts the data into Vec<u8>.
//!
//!     // Restore data
//!     let unpacked = ArrayObject::unpack(packed).unwrap();
//!     let inflated: Vec<u32> = unpacked.try_into().unwrap();
//!     assert_eq!(original, inflated);
//! }
//! ```
//!
//! One can also use the macros to write and read a file:
//! ```
//! use array_object::*;
//!
//! fn main() {
//!     // Save into a file
//!     let original = vec![1f64, 2.2, -1.1, 5.6];
//!     export_obj!("testdata.bin", original.clone()); // The type has to be known at this point.
//!
//!     // Load from a file
//!     let restored: Vec<f64> = import_obj!("testdata.bin"); // The type annotation is required.
//!     assert_eq!(original, restored);
//! }
//! ```

/// Adaptors for Complex and Array. These can be used to restore the data or construct ArrayObject without num::complex, ndarray or nalgebra.
pub mod adaptor;
mod bitfield;
mod convert;
mod error;
mod external;
mod misc;
mod pack;
mod storage;

pub use misc::TryConcat;
pub use pack::Pack;
pub use pack::Unpack;
pub use storage::{ArrayObject, DataType};
