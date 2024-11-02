use crate::DataType;

pub const TYPE_MASK: u8 = 0b_111_0_0000;

pub const SHORT_UNSIGNED_INTEGER: u8 = 0b_000_0_0000;
pub const SHORT_SIGNED_INTEGER: u8 = 0b_001_0_0000;
pub const UNSIGNED_INTEGER: u8 = 0b_010_0_0000;
pub const SIGNED_INTEGER: u8 = 0b_011_0_0000;
pub const REAL: u8 = 0b_100_0_0000;
pub const COMPLEX: u8 = 0b_101_0_0000;
pub const STRING: u8 = 0b_110_0_0000;

pub const SHORTDATA_MASK: u8 = 0b_000_1_1111;

pub const DIMENSION_MASK: u8 = 0b_000_0_1111;

pub const FORMAT_MASK: u8 = 0b_000_1_0000;

pub const FIXED_LENGTH: u8 = 0b_000_0_0000;
pub const VARIABLE_LENGTH: u8 = 0b_000_1_0000;

pub const JOINED: u8 = 0b_000_0_0000;
pub const DICTIONARY: u8 = 0b_000_1_0000;

impl DataType {
    /// Reads the last byte of binary data and describes the data type.
    pub fn describe_footer(binary: &Vec<u8>) -> String {
        let footer = binary.last().unwrap();
        match footer & TYPE_MASK {
            SHORT_UNSIGNED_INTEGER => "Short unsigned integer".to_string(),
            UNSIGNED_INTEGER => {
                if footer & FORMAT_MASK == VARIABLE_LENGTH {
                    format!(
                        "{}-dimensional variable length unsigned integer",
                        footer & DIMENSION_MASK
                    )
                } else {
                    format!(
                        "{}-dimensional fixed length unsigned integer",
                        footer & DIMENSION_MASK
                    )
                }
            }
            SHORT_SIGNED_INTEGER => "Short signed integer".to_string(),
            SIGNED_INTEGER => {
                if footer & FORMAT_MASK == VARIABLE_LENGTH {
                    format!(
                        "{}-dimensional variable length signed integer",
                        footer & DIMENSION_MASK
                    )
                } else {
                    format!(
                        "{}-dimensional fixed length signed integer",
                        footer & DIMENSION_MASK
                    )
                }
            }
            REAL => {
                if footer & FORMAT_MASK == VARIABLE_LENGTH {
                    format!(
                        "{}-dimensional variable length real number",
                        footer & DIMENSION_MASK
                    )
                } else {
                    format!(
                        "{}-dimensional fixed length real number",
                        footer & DIMENSION_MASK
                    )
                }
            }
            COMPLEX => {
                if footer & FORMAT_MASK == VARIABLE_LENGTH {
                    format!(
                        "{}-dimensional variable length complex number",
                        footer & DIMENSION_MASK
                    )
                } else {
                    format!(
                        "{}-dimensional fixed length complex number",
                        footer & DIMENSION_MASK
                    )
                }
            }
            STRING => {
                if footer & FORMAT_MASK == DICTIONARY {
                    format!(
                        "{}-dimensional compressed string with dictionary",
                        footer & DIMENSION_MASK
                    )
                } else {
                    format!("{}-dimensional joined string", footer & DIMENSION_MASK)
                }
            }
            _ => {
                panic!();
            }
        }
    }
}