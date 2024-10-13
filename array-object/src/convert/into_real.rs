use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::storage::*;

macro_rules! into_float {
    ($($ty:tt),*) => {
        $(
            impl TryFrom<ArrayObject> for $ty {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if !val.shape.is_empty() || val.datatype != DataType::Real {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    match val.data.len() {
                        4 => {
                            let data = f32::from_le_bytes(val.data.try_into().unwrap()) as $ty;
                            Ok(data)
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            let data = f64::from_le_bytes(val.data.try_into().unwrap()) as $ty;
                            Ok(data)
                        }
                        _ => {panic!();}
                    }
                }
            }
            impl TryFrom<ArrayObject> for VecShape<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.is_empty() || val.datatype != DataType::Real {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let len = val.len();
                    match val.data.len() / len {
                        4 => {
                            let data = val.data.chunks(4).map(|b| f32::from_le_bytes(b.try_into().unwrap()) as $ty).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            let data = val.data.chunks(8).map(|b| f64::from_le_bytes(b.try_into().unwrap()) as $ty).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        _ => {panic!();}
                    }
                }
            }
            impl TryFrom<ArrayObject> for Vec<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.len() != 1 {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let VecShape::<$ty>(data, _) = val.try_into()?;
                    Ok(data)
                }
            }
        )*
    };
}

into_float!(f32, f64);
