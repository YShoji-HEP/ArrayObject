use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::misc::Product;
use crate::storage::*;

macro_rules! from_float {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for ArrayObject {
                fn from(val: $ty) -> Self {
                    let data = val.to_le_bytes().to_vec();
                    Self {
                        data,
                        shape: vec![],
                        datatype: DataType::Real,
                    }
                }
            }
            impl From<&$ty> for ArrayObject {
                fn from(val: &$ty) -> Self {
                    let data = val.to_le_bytes().to_vec();
                    Self {
                        data,
                        shape: vec![],
                        datatype: DataType::Real,
                    }
                }
            }
            impl From<Vec<$ty>> for ArrayObject {
                fn from(val: Vec<$ty>) -> Self {
                    let shape = vec![val.len() as u64];
                    let mut data = Vec::<u8>::with_capacity(size_of::<$ty>() * val.len());
                    for v in val {
                        data.append(&mut v.to_le_bytes().to_vec());
                    }
                    Self {
                        data,
                        shape,
                        datatype: DataType::Real,
                    }
                }
            }
            impl From<&Vec<$ty>> for ArrayObject {
                fn from(val: &Vec<$ty>) -> Self {
                    let shape = vec![val.len() as u64];
                    let mut data = Vec::<u8>::with_capacity(size_of::<$ty>() * val.len());
                    for v in val {
                        data.append(&mut v.to_le_bytes().to_vec());
                    }
                    Self {
                        data,
                        shape,
                        datatype: DataType::Real,
                    }
                }
            }
            impl<const N: usize> From<[$ty; N]> for ArrayObject {
                fn from(val: [$ty; N]) -> Self {
                    val.to_vec().into()
                }
            }
            impl From<&[$ty]> for ArrayObject {
                fn from(val: &[$ty]) -> Self {
                    val.to_vec().into()
                }
            }
            impl TryFrom<VecShape<$ty>> for ArrayObject {
                type Error = ArrayObjectError;
                fn try_from(VecShape(val, shape): VecShape<$ty>) -> Result<Self, Self::Error> {
                    if val.len() != shape.product() as usize {
                        return Err(ArrayObjectError::NumberOfElementsMismatch(val.len(), shape.product() as usize));
                    }
                    if shape.len() > 15 {
                        return Err(ArrayObjectError::TooLargeDimension(shape.len()));
                    }
                    let mut temp: ArrayObject = val.into();
                    temp.shape = shape;
                    Ok(temp)
                }
            }
        )*
    };
}

from_float!(f32, f64);
