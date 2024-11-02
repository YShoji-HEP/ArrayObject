use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::misc::Product;
use crate::storage::*;
use num_complex::Complex;

macro_rules! from_complex {
    ($($ty:ty),*) => {
        $(
            impl From<Pair<$ty>> for ArrayObject {
                fn from(Pair(val_re, val_im): Pair<$ty>) -> Self {
                    let data = [val_re.to_le_bytes().to_vec(), val_im.to_le_bytes().to_vec()].concat();
                    Self {
                        data,
                        shape: vec![],
                        datatype: DataType::Complex,
                    }
                }
            }
            impl From<Complex<$ty>> for ArrayObject {
                fn from(val: Complex<$ty>) -> Self {
                    Pair(val.re, val.im).into()
                }
            }
            impl<const N: usize> From<[Complex<$ty>; N]> for ArrayObject {
                fn from(val: [Complex<$ty>; N]) -> Self {
                    val.to_vec().into()
                }
            }
            impl From<&[Complex<$ty>]> for ArrayObject {
                fn from(val: &[Complex<$ty>]) -> Self {
                    val.to_vec().into()
                }
            }
            impl From<Vec<Pair<$ty>>> for ArrayObject {
                fn from(val: Vec<Pair<$ty>>) -> Self {
                    let shape = vec![val.len() as u64];
                    let mut data = Vec::<u8>::with_capacity(2 * val.len() * size_of::<$ty>());
                    for v in val {
                        let Pair(re, im) = v;
                        data.append(&mut re.to_le_bytes().to_vec());
                        data.append(&mut im.to_le_bytes().to_vec());
                    }
                    Self {
                        data,
                        shape,
                        datatype: DataType::Complex,
                    }
                }
            }
            impl From<Vec<Complex<$ty>>> for ArrayObject {
                fn from(val: Vec<Complex<$ty>>) -> Self {
                    let shape = vec![val.len() as u64];
                    let mut data = Vec::<u8>::with_capacity(2 * val.len() * size_of::<$ty>());
                    for v in val {
                        data.append(&mut v.re.to_le_bytes().to_vec());
                        data.append(&mut v.im.to_le_bytes().to_vec());
                    }
                    Self {
                        data,
                        shape,
                        datatype: DataType::Complex,
                    }
                }
            }
            impl TryFrom<VecVec<$ty>> for ArrayObject {
                type Error = ArrayObjectError;
                fn try_from(VecVec(val_re, val_im): VecVec<$ty>) -> Result<Self, Self::Error> {
                    if val_re.len() != val_im.len() {
                        return Err(ArrayObjectError::VectorLengthMismatch(val_re.len(), val_im.len()));
                    }
                    let shape = vec![val_re.len() as u64];
                    let mut data = Vec::<u8>::with_capacity(2 * val_re.len() * size_of::<$ty>());
                    for (re, im) in val_re.into_iter().zip(val_im.into_iter()) {
                        data.append(&mut re.to_le_bytes().to_vec());
                        data.append(&mut im.to_le_bytes().to_vec());
                    }
                    Ok(Self {
                        data,
                        shape,
                        datatype: DataType::Complex,
                    })
                }
            }
            impl TryFrom<VecShape<Complex<$ty>>> for ArrayObject {
                type Error = ArrayObjectError;
                fn try_from(VecShape(val, shape): VecShape<Complex<$ty>>) -> Result<Self, Self::Error> {
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
            impl TryFrom<VecVecShape<$ty>> for ArrayObject {
                type Error = ArrayObjectError;
                fn try_from(VecVecShape(val_re, val_im, shape): VecVecShape<$ty>) -> Result<Self, Self::Error> {
                    if val_re.len() != shape.product() as usize {
                        return Err(ArrayObjectError::NumberOfElementsMismatch(val_re.len(), shape.product() as usize));
                    }
                    if shape.len() > 15 {
                        return Err(ArrayObjectError::TooLargeDimension(shape.len()));
                    }
                    let mut temp: ArrayObject = VecVec(val_re, val_im).try_into()?;
                    temp.shape = shape;
                    Ok(temp)
                }
            }
        )*
    };
}

from_complex!(f32, f64);
