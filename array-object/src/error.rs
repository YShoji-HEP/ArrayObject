use std::error::Error;
use std::fmt::{Debug, Display};

use crate::storage::DataType;

pub enum ArrayObjectError {
    VectorLengthMismatch(usize, usize),
    NumberOfElementsMismatch(usize, usize),
    TooLargeDimension(usize),
    WrongDataType(DataType, usize),
    LossyConversion,
    ConcatShapeMismatch,
    UnableToDecode,
    IncompatibleConversion(bool, usize),
    External(&'static str),
}

impl Display for ArrayObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayObjectError::VectorLengthMismatch(len_re, len_im) => {
                write!(
                    f,
                    "Two vectors should have the same length: {len_re} != {len_im}."
                )
            }
            ArrayObjectError::NumberOfElementsMismatch(vec_len, total_len) => {
                write!(f, "The length of the vector should agree with the number of elements in the array: expected {total_len}, found {vec_len}.")
            }
            ArrayObjectError::TooLargeDimension(dim) => {
                write!(
                    f,
                    "The dimension of the array should be smaller than 16: found {dim}."
                )
            }
            ArrayObjectError::WrongDataType(ty, dim) => match dim {
                0 => write!(f, "The wrong data type: expected [Single value of {ty:?}]."),
                _ => write!(f, "The wrong data type: expected [{dim}D Array of {ty:?}]."),
            },
            ArrayObjectError::LossyConversion => {
                write!(f, "The lossy conversion of float numbers is disabled by default. See [features] allow_float_down_convert.")
            }
            ArrayObjectError::ConcatShapeMismatch => {
                write!(f, "The concatenation of the ArrayObject is only allowed for the ones having the same size, shape and type.")
            }
            ArrayObjectError::UnableToDecode => {
                write!(f, "The data is either broken or wrong.")
            }
            ArrayObjectError::IncompatibleConversion(is_signed, bits) => {
                if *is_signed {
                    write!(
                        f,
                        "The integer type is imcompatible with the data. Required: i{bits}"
                    )
                } else {
                    write!(
                        f,
                        "The integer type is imcompatible with the data. Required: u{bits}"
                    )
                }
            }
            ArrayObjectError::External(err) => {
                write!(f, "{err}")
            }
        }
    }
}

impl Debug for ArrayObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ArrayObjectError {}
