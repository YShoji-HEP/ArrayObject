use crate::misc::Product;

/// The type of the array.
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    UnsignedInteger,
    SignedInteger,
    Real,
    Complex,
    String,
}

/// The main array storage.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayObject {
    pub(crate) data: Vec<u8>,
    pub(crate) shape: Vec<u64>,
    pub(crate) datatype: DataType,
}

impl ArrayObject {
    /// Returens the minimal size, in bits, required to restore the array.
    pub fn bits(&self) -> Option<usize> {
        match self.datatype {
            DataType::UnsignedInteger | DataType::SignedInteger | DataType::Real => {
                Some(8 * self.data.len() / self.shape.product() as usize)
            }
            DataType::Complex => Some(8 * self.data.len() / self.shape.product() as usize / 2),
            DataType::String => None,
        }
    }
    /// Returns the total number of elements in the array.
    pub fn len(&self) -> usize {
        self.shape.product() as usize
    }
    /// Returns the shape of the array.
    pub fn shape(&self) -> Vec<usize> {
        self.shape.iter().map(|&x| x as usize).collect()
    }
    /// Returns the uncompressed datasize.
    pub fn datasize(&self) -> usize {
        self.data.len()
    }
    /// Returns the data type.
    pub fn datatype(&self) -> DataType {
        self.datatype.clone()
    }
    /// Returns the dimension of the array.
    pub fn dimension(&self) -> usize {
        self.shape.len()
    }
}

pub const TYPE_MASK: u8 = 0b_111_0_0000;

pub const SHORT_UNSIGNED_INTEGER: u8 = 0b_000_0_0000;
pub const SHORT_SIGNED_INTEGER: u8 = 0b_001_0_0000;
pub const UNSIGNED_INTEGER: u8 = 0b_010_0_0000;
pub const SIGNED_INTEGER: u8 = 0b_011_0_0000;
pub const REAL: u8 = 0b_100_0_0000;
pub const COMPLEX: u8 = 0b_101_0_0000;
pub const STRING: u8 = 0b_110_0_0000;

pub const SHORTDATA_MASK: u8 = 0b_000_1_1111;

pub const DIMENSION_MASK: u8 = 0b_000_0_1111;

pub const FORMAT_MASK: u8 = 0b_000_1_0000;

pub const FIXED_LENGTH: u8 = 0b_000_0_0000;
pub const VARIABLE_LENGTH: u8 = 0b_000_1_0000;

pub const JOINED: u8 = 0b_000_0_0000;
pub const DICTIONARY: u8 = 0b_000_1_0000;
