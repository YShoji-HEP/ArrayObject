use crate::misc::Product;

/// The type of the elements.
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    UnsignedInteger,
    SignedInteger,
    Real,
    Complex,
    String,
}

/// The main array storage with type abstraction.
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
