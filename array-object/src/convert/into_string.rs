use crate::adaptor::*;
use crate::error::ArrayObjectError;
use crate::storage::*;

impl TryFrom<ArrayObject> for String {
    type Error = ArrayObjectError;
    fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
        if !val.shape.is_empty() || val.datatype != DataType::String {
            return Err(ArrayObjectError::WrongDataType(
                val.datatype,
                val.shape.len(),
            ));
        }
        Ok(String::from_utf8(val.data).unwrap())
    }
}

impl TryFrom<ArrayObject> for VecShape<String> {
    type Error = ArrayObjectError;
    fn try_from(mut val: ArrayObject) -> Result<Self, Self::Error> {
        if val.shape.is_empty() || val.datatype != DataType::String {
            return Err(ArrayObjectError::WrongDataType(
                val.datatype,
                val.shape.len(),
            ));
        }
        if val.len() == 0 {
            return Ok(VecShape(vec![], val.shape));
        }
        let mut data = vec![];
        while let Some(p) = val.data.iter().position(|&x| x == 255) {
            let mut s = val.data.split_off(p + 1);
            std::mem::swap(&mut s, &mut val.data);
            s.pop();
            data.push(String::from_utf8(s).unwrap());
        }
        data.push(String::from_utf8(val.data).unwrap());
        Ok(VecShape(data, val.shape))
    }
}

impl TryFrom<ArrayObject> for Vec<String> {
    type Error = ArrayObjectError;
    fn try_from(val: ArrayObject) -> Result<Self, Self::Error> {
        if val.shape.len() != 1 {
            return Err(ArrayObjectError::WrongDataType(
                val.datatype,
                val.shape.len(),
            ));
        }
        let VecShape::<String>(data, _) = val.try_into()?;
        Ok(data)
    }
}
