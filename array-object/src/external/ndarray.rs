#[cfg(feature = "ndarray_15")]
use ndarray_15 as ndarray;
#[cfg(feature = "ndarray_16")]
use ndarray_16 as ndarray;

use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::ArrayObject;
use ndarray::{Array, Array0, Array1, Array2, Array3, Array4, Array5, Array6, ArrayD, Dimension};
use num_complex::Complex;

macro_rules! ndarray_impl {
    ($($ty:ty),*) => {
        $(
            impl<D: Dimension> TryFrom<Array<$ty, D>> for ArrayObject {
                type Error = ArrayObjectError;
                fn try_from(val: Array<$ty, D>) -> Result<Self, Self::Error> {
                    let shape: Vec<_> = val.shape().iter().map(|x| *x as u64).collect();
                    let v: Vec<_> = val.into_iter().collect();
                    VecShape(v, shape).try_into()
                }
            }
            impl TryFrom<ArrayObject> for ArrayD<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    let temp = Array::from_shape_vec(shape.as_slice(), data).unwrap();
                    Ok(temp)
                }
            }
            impl TryFrom<ArrayObject> for Array0<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(mut data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.is_empty() {
                        let temp = Array0::from_elem([], data.pop().unwrap());
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array1<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 1 {
                        let temp = Array1::from_shape_vec(shape[0], data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array2<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 2 {
                        let temp = Array2::from_shape_vec((shape[0], shape[1]), data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array3<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 3 {
                        let temp = Array3::from_shape_vec((shape[0], shape[1], shape[2]), data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array4<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 4 {
                        let temp = Array4::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array5<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 5 {
                        let temp = Array5::from_shape_vec((shape[0], shape[1], shape[2], shape[3], shape[4]), data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
            impl TryFrom<ArrayObject> for Array6<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.len() == 6 {
                        let temp = Array6::from_shape_vec((shape[0], shape[1], shape[2], shape[3], shape[4], shape[5]), data).unwrap();
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("Array dimension mismatch."))
                    }
                }
            }
        )*
    };
}

ndarray_impl!(
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    String,
    Complex<f32>,
    Complex<f64>
);
