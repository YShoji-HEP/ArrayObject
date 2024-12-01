use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::storage::RawStorage;
use nalgebra::base::{DMatrix, DVector, Matrix};
use nalgebra::Complex;

use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::ArrayObject;

macro_rules! nalgebra_impl {
    ($($ty:ty),*) => {
        $(
            impl<R: Dim, C: Dim, S: RawStorage<$ty, R, C>> TryFrom<Matrix<$ty, R, C, S>> for ArrayObject where DefaultAllocator: Allocator<C, R> {
                type Error = ArrayObjectError;
                fn try_from(val: Matrix<$ty, R, C, S>) -> Result<Self, Self::Error> {
                    let shape = vec![val.shape().0 as u64, val.shape().1 as u64];
                    let v: Vec<_> = val.transpose().iter().copied().collect();
                    VecShape(v, shape).try_into()
                }
            }
            impl TryFrom<ArrayObject> for DMatrix<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    match shape.len() {
                        1 => {
                            let temp = DMatrix::from_vec(1, shape[0] as usize, data);
                            Ok(temp.transpose())
                        },
                        2 => {
                            let temp = DMatrix::from_vec(shape[1] as usize, shape[0] as usize, data);
                            Ok(temp.transpose())
                        },
                        _ => Err(ArrayObjectError::External("The data is not a matrix or a vector"))
                    }
                }
            }
            impl TryFrom<ArrayObject> for DVector<$ty> {
                type Error = ArrayObjectError;
                fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
                    let VecShape::<$ty>(data, shape) = val.try_into()?;
                    if shape.len() == 1 {
                        let temp = DVector::from_vec(data);
                        Ok(temp)
                    } else {
                        Err(ArrayObjectError::External("The data is not a vector"))}
                }
            }
        )*
    };
}

nalgebra_impl!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    Complex<f32>,
    Complex<f64>
);
