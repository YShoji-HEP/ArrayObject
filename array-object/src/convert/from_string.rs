use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::misc::Product;
use crate::storage::*;

macro_rules! from_text {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for ArrayObject {
                fn from(val: $ty) -> Self {
                    let data: Vec<u8> = val.to_string().into_bytes().to_vec();
                    Self {
                        data,
                        shape: vec![],
                        datatype: DataType::String,
                    }
                }
            }
            impl From<&$ty> for ArrayObject {
                fn from(val: &$ty) -> Self {
                    let data: Vec<u8> = val.to_string().into_bytes().to_vec();
                    Self {
                        data,
                        shape: vec![],
                        datatype: DataType::String,
                    }
                }
            }
            impl From<Vec<$ty>> for ArrayObject {
                fn from(val: Vec<$ty>) -> Self {
                    let shape = vec![val.len() as u64];
                    let val: Vec<_> = val.into_iter().map(|x| x.to_string()).collect();
                    let data = val.into_iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>().join(&255u8);
                    Self {
                        data,
                        shape,
                        datatype: DataType::String,
                    }
                }
            }
            impl From<&Vec<$ty>> for ArrayObject {
                fn from(val: &Vec<$ty>) -> Self {
                    let shape = vec![val.len() as u64];
                    let val: Vec<_> = val.into_iter().map(|x| x.to_string()).collect();
                    let data = val.into_iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>().join(&255u8);
                    Self {
                        data,
                        shape,
                        datatype: DataType::String,
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

from_text!(String, &str);
