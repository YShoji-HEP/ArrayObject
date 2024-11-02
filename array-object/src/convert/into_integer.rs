use crate::adaptor::*;
use crate::convert::zigzag::Zigzag;
use crate::error::ArrayObjectError;
use crate::storage::*;

macro_rules! into_integer {
    ($($ty:tt),*) => {
        $(
            impl TryFrom<ArrayObject> for $ty {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if !val.shape.is_empty() {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    if val.datatype == DataType::UnsignedInteger {
                        match val.data.len() {
                            1 => {
                                let data = u8::from_le_bytes(val.data.try_into().unwrap()).try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(false, 8)))?;
                                Ok(data)
                            }
                            2 => {
                                let data = u16::from_le_bytes(val.data.try_into().unwrap()).try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(false, 16)))?;
                                Ok(data)
                            }
                            4 => {
                                let data = u32::from_le_bytes(val.data.try_into().unwrap()).try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(false, 32)))?;
                                Ok(data)
                            }
                            8 => {
                                let data = u64::from_le_bytes(val.data.try_into().unwrap()).try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(false, 64)))?;
                                Ok(data)
                            }
                            16 => {
                                let data = u128::from_le_bytes(val.data.try_into().unwrap()).try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(false, 128)))?;
                                Ok(data)
                            }
                            _ => {panic!();}
                        }
                    } else if val.datatype == DataType::SignedInteger {
                        match val.data.len() {
                            1 => {
                                let data = i8::from_le_bytes(val.data.try_into().unwrap()).straight().try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(true, 8)))?;
                                Ok(data)
                            }
                            2 => {
                                let data = i16::from_le_bytes(val.data.try_into().unwrap()).straight().try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(true, 16)))?;
                                Ok(data)
                            }
                            4 => {
                                let data = i32::from_le_bytes(val.data.try_into().unwrap()).straight().try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(true, 32)))?;
                                Ok(data)
                            }
                            8 => {
                                let data = i64::from_le_bytes(val.data.try_into().unwrap()).straight().try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(true, 64)))?;
                                Ok(data)
                            }
                            16 => {
                                let data = i128::from_le_bytes(val.data.try_into().unwrap()).straight().try_into()
                                .or(Err(ArrayObjectError::IncompatibleConversion(true, 128)))?;
                                Ok(data)
                            }
                            _ => {panic!();}
                        }
                    } else {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                }
            }
            impl TryFrom<ArrayObject> for VecShape<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.is_empty() {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let len = val.len();
                    if len == 0 {
                        return Ok(VecShape(vec![], val.shape));
                    }
                    if val.datatype == DataType::UnsignedInteger {
                        match val.data.len() / len {
                            1 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(1) {
                                    data.push(u8::from_le_bytes(b.try_into().unwrap()).try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(false, 8)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            2 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(2) {
                                    data.push(u16::from_le_bytes(b.try_into().unwrap()).try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(false, 16)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            4 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(4) {
                                    data.push(u32::from_le_bytes(b.try_into().unwrap()).try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(false, 32)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            8 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(8) {
                                    data.push(u64::from_le_bytes(b.try_into().unwrap()).try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(false, 64)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            16 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(16) {
                                    data.push(u128::from_le_bytes(b.try_into().unwrap()).try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(false, 128)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            _ => {panic!();}
                        }
                    } else if val.datatype == DataType::SignedInteger {
                        match val.data.len() / len {
                            1 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(1) {
                                    data.push(i8::from_le_bytes(b.try_into().unwrap()).straight().try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(true, 8)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            2 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(2) {
                                    data.push(i16::from_le_bytes(b.try_into().unwrap()).straight().try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(true, 16)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            4 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(4) {
                                    data.push(i32::from_le_bytes(b.try_into().unwrap()).straight().try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(true, 32)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            8 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(8) {
                                    data.push(i64::from_le_bytes(b.try_into().unwrap()).straight().try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(true, 64)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            16 => {
                                let mut data = Vec::with_capacity(len);
                                for b in val.data.chunks(16) {
                                    data.push(i128::from_le_bytes(b.try_into().unwrap()).straight().try_into()
                                    .or(Err(ArrayObjectError::IncompatibleConversion(true, 128)))?);
                                }
                                Ok(VecShape(data, val.shape))
                            }
                            _ => {panic!();}
                        }
                    } else {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
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
            impl<const N: usize> TryFrom<ArrayObject> for [$ty; N] {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.len() != N {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let data: Vec<$ty> = val.try_into()?;
                    Ok(data.try_into().unwrap())
                }
            }
        )*
    }
}

into_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
