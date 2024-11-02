use crate::bitfield::*;
use crate::error::ArrayObjectError;
use crate::misc::Product;
use crate::pack::unpack_float::*;
use crate::pack::unpack_integer::*;
use crate::pack::unpack_string::*;
use crate::pack::varint::*;
use crate::storage::*;

/// Restore from binary.
pub trait Unpack {
    /// Restore ArrayObject from a binary data.
    fn unpack(data: Vec<u8>) -> Result<Self, ArrayObjectError>
    where
        Self: Sized;
}

impl Unpack for ArrayObject {
    fn unpack(mut data: Vec<u8>) -> Result<Self, ArrayObjectError> {
        let (datatype, format, shape, shortdata) = read_footer(&mut data);
        match datatype & TYPE_MASK {
            SHORT_UNSIGNED_INTEGER => {
                if !data.is_empty() {
                    return Err(ArrayObjectError::UnableToDecode);
                }
                Ok(Self {
                    data: shortdata.unwrap(),
                    shape: vec![],
                    datatype: DataType::UnsignedInteger,
                })
            }
            SHORT_SIGNED_INTEGER => {
                if !data.is_empty() {
                    return Err(ArrayObjectError::UnableToDecode);
                }
                Ok(Self {
                    data: shortdata.unwrap(),
                    shape: vec![],
                    datatype: DataType::SignedInteger,
                })
            }
            UNSIGNED_INTEGER => {
                let shape = shape.unwrap();
                if format == VARIABLE_LENGTH {
                    data = from_variable_integer(data);
                } else {
                    let total_len = shape.product();
                    while (data.len() == 0 && total_len > 0)
                        || (total_len == 1 && 2usize.pow(data.len().ilog2()) != data.len())
                    {
                        data.push(0);
                    }
                }
                Ok(Self {
                    data,
                    shape,
                    datatype: DataType::UnsignedInteger,
                })
            }
            SIGNED_INTEGER => {
                let shape = shape.unwrap();
                if format == VARIABLE_LENGTH {
                    data = from_variable_integer(data);
                } else {
                    let total_len = shape.product();
                    while (data.len() == 0 && total_len > 0)
                        || (total_len == 1 && 2usize.pow(data.len().ilog2()) != data.len())
                    {
                        data.push(0);
                    }
                }
                Ok(Self {
                    data,
                    shape,
                    datatype: DataType::SignedInteger,
                })
            }
            REAL => {
                if format == VARIABLE_LENGTH {
                    data = from_variable_float(data);
                }
                Ok(Self {
                    data,
                    shape: shape.unwrap(),
                    datatype: DataType::Real,
                })
            }
            COMPLEX => {
                if format == VARIABLE_LENGTH {
                    data = from_variable_float(data);
                }
                Ok(Self {
                    data,
                    shape: shape.unwrap(),
                    datatype: DataType::Complex,
                })
            }
            STRING => {
                if format == DICTIONARY {
                    data = from_dictionary(data);
                }
                Ok(Self {
                    data,
                    shape: shape.unwrap(),
                    datatype: DataType::String,
                })
            }
            _ => {
                return Err(ArrayObjectError::UnableToDecode);
            }
        }
    }
}

fn read_footer(bytes: &mut Vec<u8>) -> (u8, u8, Option<Vec<u64>>, Option<Vec<u8>>) {
    let last = bytes.pop().unwrap();
    let ty = last & TYPE_MASK;
    let format = last & FORMAT_MASK;
    if ty == SHORT_UNSIGNED_INTEGER || ty == SHORT_SIGNED_INTEGER {
        let data = last & SHORTDATA_MASK;
        (ty, format, None, Some(vec![data]))
    } else {
        let dim = (last & DIMENSION_MASK) as usize;
        let iter = bytes.iter().rev();
        let (shape, len) = varint_decode(iter, dim);
        bytes.truncate(bytes.len() - len);
        (ty, format, Some(shape), None)
    }
}
