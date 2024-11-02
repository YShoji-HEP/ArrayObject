use crate::error::ArrayObjectError;
use crate::ArrayObject;

pub(crate) trait Product {
    fn product(&self) -> u64;
}

impl Product for Vec<u64> {
    fn product(&self) -> u64 {
        self.iter().product::<u64>()
    }
}

/// Concat ArrayObjects of the same type, size and shape.
///
/// ```
/// use array_object::{ArrayObject, TryConcat};
/// let mut objs = vec![];
/// for _ in 0..8 {
///     let temp: ArrayObject = vec![0i32, 1, 2, 3].into();
///     objs.push(temp);
/// }
/// let obj = objs.try_concat().unwrap();
/// ```
pub trait TryConcat {
    fn try_concat(self) -> Result<ArrayObject, ArrayObjectError>;
}

impl TryConcat for Vec<ArrayObject> {
    fn try_concat(self) -> Result<ArrayObject, ArrayObjectError> {
        if self[0].shape.len() > 14 {
            return Err(ArrayObjectError::TooLargeDimension(self[0].shape.len() + 1));
        }
        let shape_orig = self[0].shape.clone();
        let datatype = self[0].datatype.clone();
        let datasize = self[0].datasize();

        let mut shape = shape_orig.clone();
        shape.insert(0, self.len() as u64);

        let mut data = vec![];
        for mut v in self.into_iter() {
            if shape_orig == v.shape && datatype == v.datatype && datasize == v.datasize() {
                data.append(&mut v.data);
            } else {
                return Err(ArrayObjectError::ConcatShapeMismatch);
            }
        }
        Ok(ArrayObject {
            data,
            shape,
            datatype,
        })
    }
}

/// A macro to save the data into a file.
///
/// ```
/// use array_object::*;
/// let data = vec![1f64, 2.2, -1.1, 5.6];
/// export_obj!("testdata.bin", data.clone());
/// ```
#[macro_export]
macro_rules! export_obj {
    ($path:literal,$x:expr) => {{
        use array_object::Pack;
        let obj: array_object::ArrayObject = $x.try_into().unwrap();
        let data = obj.pack();
        std::fs::write($path, data).unwrap();
    }};
}

/// A macro to load the data from a file.
///
/// ```
/// use array_object::*;
/// let restored: Vec<f64> = import_obj!("testdata.bin");
/// ```
#[macro_export]
macro_rules! import_obj {
    ($path:literal) => {{
        use array_object::Unpack;
        let data = std::fs::read($path).unwrap();
        let obj = array_object::ArrayObject::unpack(data).unwrap();
        obj.try_into().unwrap()
    }};
}
