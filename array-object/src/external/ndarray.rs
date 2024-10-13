#[cfg(feature = "ndarray_15")]
use ndarray_15 as ndarray;
#[cfg(feature = "ndarray_16")]
use ndarray_16 as ndarray;

use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::ArrayObject;
use ndarray::{Array, Dim, Dimension, IxDynImpl};
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
            impl TryFrom<ArrayObject> for Array<$ty, Dim<IxDynImpl>> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    let shape: Vec<usize> = shape.iter().map(|&x| x.try_into().unwrap()).collect();
                    if shape.is_empty() {
                        Err(ArrayObjectError::External("The data is not an array."))
                    } else {
                        let temp = Array::from_shape_vec(shape.as_slice(), data).unwrap();
                        Ok(temp)
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
