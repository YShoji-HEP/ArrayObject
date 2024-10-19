use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::storage::*;
use num_complex::Complex;

macro_rules! into_complex {
    ($($ty:tt),*) => {
        $(
            impl TryFrom<ArrayObject> for Pair<$ty> {
                type Error = ArrayObjectError;
                fn try_from(mut val: ArrayObject) -> Result<Self, Self::Error> {
                    if !val.shape.is_empty() || val.datatype != DataType::Complex {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    match val.data.len() / 2 {
                        4 => {
                            let data_im = val.data.split_off(4);
                            let re = f32::from_le_bytes(val.data.try_into().unwrap()) as $ty;
                            let im = f32::from_le_bytes(data_im.try_into().unwrap()) as $ty;
                            Ok(Pair(re, im))
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            let data_im = val.data.split_off(8);
                            let re = f64::from_le_bytes(val.data.try_into().unwrap()) as $ty;
                            let im = f64::from_le_bytes(data_im.try_into().unwrap()) as $ty;
                            Ok(Pair(re, im))
                        }
                        _ => {panic!();}
                    }
                }
            }
            impl TryFrom<ArrayObject> for Complex<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let Pair(re, im) = val.try_into()?;
                    Ok(Complex::new(re, im))
                }
            }
            impl TryFrom<ArrayObject> for VecShape<Pair<$ty>> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.is_empty() || val.datatype != DataType::Complex {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let len = val.len();
                    if len == 0 {
                        return Ok(VecShape(vec![], val.shape));
                    }
                    match val.data.len() / (2 * len) {
                        4 => {
                            let data = val.data.chunks(8).map(|b| {
                                let mut iter = b.chunks(4);
                                let re = f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                let im = f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                Pair(re, im)
                            }).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            let data = val.data.chunks(16).map(|b| {
                                let mut iter = b.chunks(8);
                                let re = f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                let im = f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                Pair(re, im)
                            }).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        _ => {panic!();}
                    }
                }
            }
            impl TryFrom<ArrayObject> for VecShape<Complex<$ty>> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.is_empty() || val.datatype != DataType::Complex {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let len = val.len();
                    if len == 0 {
                        return Ok(VecShape(vec![], val.shape));
                    }
                    match val.data.len() / (2 * len) {
                        4 => {
                            let data = val.data.chunks(8).map(|b| {
                                let mut iter = b.chunks(4);
                                let re = f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                let im = f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                Complex::<$ty>::new(re, im)
                            }).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            let data = val.data.chunks(16).map(|b| {
                                let mut iter = b.chunks(8);
                                let re = f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                let im = f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty;
                                Complex::<$ty>::new(re, im)
                            }).collect();
                            Ok(VecShape(data, val.shape))
                        }
                        _ => {panic!();}
                    }
                }
            }
            impl TryFrom<ArrayObject> for VecVecShape<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.is_empty() || val.datatype != DataType::Complex {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let len = val.len();
                    if len == 0 {
                        return Ok(VecVecShape(vec![], vec![], val.shape));
                    }
                    let mut re = Vec::<$ty>::with_capacity(len * 2);
                    let mut im = Vec::<$ty>::with_capacity(len * 2);
                    match val.data.len() / (2 * len) {
                        4 => {
                            for b in val.data.chunks(8) {
                                let mut iter = b.chunks(4);
                                re.push(f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty);
                                im.push(f32::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty);
                            }
                        }
                        8 => {
                            #[cfg(not(feature = "allow_float_down_convert"))]
                            if size_of::<$ty>() < 8 {
                                return Err(ArrayObjectError::LossyConversion);
                            }
                            for b in val.data.chunks(16) {
                                let mut iter = b.chunks(8);
                                re.push(f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty);
                                im.push(f64::from_le_bytes(iter.next().unwrap().to_vec().try_into().unwrap()) as $ty);
                            }
                        }
                        _ => {panic!();}
                    }
                    Ok(VecVecShape(re, im, val.shape))
                }
            }
            impl TryFrom<ArrayObject> for Vec<Complex<$ty>> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    if val.shape.len() != 1 {
                        return Err(ArrayObjectError::WrongDataType(val.datatype, val.shape.len()));
                    }
                    let VecShape::<Complex<$ty>>(data, _) = val.try_into()?;
                    Ok(data)
                }
            }
        )*
    };
}

into_complex!(f32, f64);
