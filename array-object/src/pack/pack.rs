use crate::misc::Product;
use crate::pack::pack_float::*;
use crate::pack::pack_integer::*;
use crate::pack::pack_string::*;
use crate::pack::varint::*;
use crate::storage::*;

/// Convert into binary.
pub trait Pack {
    /// Compress the data and create a binary object.
    fn pack(self) -> Vec<u8>;
    /// Create a binary object without compression.
    fn pack_as_it_is(self) -> Vec<u8>;
}

impl Pack for ArrayObject {
    fn pack(self) -> Vec<u8> {
        match self.datatype {
            DataType::UnsignedInteger => {
                let len = self.shape.product();
                if len == 0 {
                    let mut data = vec![];
                    let datatype = UNSIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    return data;
                }
                let size_orig = self.data.len() / len as usize;
                match inspect_integer(&self.data, size_orig, &self.shape) {
                    IntegerPackingOption::FixedLength(size_new) => {
                        let mut data = into_fixed_integer(self.data, size_orig, size_new);
                        let datatype = UNSIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::VariableLength(total_len) => {
                        let mut data = into_variable_integer(self.data, size_orig, total_len);
                        let datatype = UNSIGNED_INTEGER | VARIABLE_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::Short => {
                        let mut data = vec![];
                        let datatype = SHORT_UNSIGNED_INTEGER | self.data[0];
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::ShortVariable => {
                        let mut data = into_short_variable_integer(self.data);
                        let datatype = UNSIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::None => {
                        let mut data = self.data;
                        let datatype = UNSIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                }
            }
            DataType::SignedInteger => {
                let len = self.shape.product();
                if len == 0 {
                    let mut data = vec![];
                    let datatype = SIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    return data;
                }
                let size_orig = self.data.len() / len as usize;
                match inspect_integer(&self.data, size_orig, &self.shape) {
                    IntegerPackingOption::FixedLength(size_new) => {
                        let mut data = into_fixed_integer(self.data, size_orig, size_new);
                        let datatype = SIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::VariableLength(total_len) => {
                        let mut data = into_variable_integer(self.data, size_orig, total_len);
                        let datatype = SIGNED_INTEGER | VARIABLE_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::Short => {
                        let mut data = vec![];
                        let datatype = SHORT_SIGNED_INTEGER | self.data[0];
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::ShortVariable => {
                        let mut data = into_short_variable_integer(self.data);
                        let datatype = SIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    IntegerPackingOption::None => {
                        let mut data = self.data;
                        let datatype = SIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                }
            }
            DataType::Real => {
                let len = self.shape.product();
                if len == 0 {
                    let mut data = vec![];
                    let datatype = REAL | FIXED_LENGTH | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    return data;
                }
                let size_orig = self.data.len() / len as usize;
                match inspect_float(&self.data, size_orig) {
                    FloatPackingOption::FixedLength(size_new) => {
                        let mut data = into_fixed_float(self.data, size_orig, size_new);
                        let datatype = REAL | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    FloatPackingOption::VariableLength(total_len) => {
                        let mut data = into_variable_float(self.data, size_orig, total_len);
                        let datatype = REAL | VARIABLE_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    FloatPackingOption::None => {
                        let mut data = self.data;
                        let datatype = REAL | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                }
            }
            DataType::Complex => {
                let len = self.shape.product();
                if len == 0 {
                    let mut data = vec![];
                    let datatype = COMPLEX | FIXED_LENGTH | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    return data;
                }
                let size_orig = self.data.len() / len as usize / 2;
                match inspect_float(&self.data, size_orig) {
                    FloatPackingOption::FixedLength(size_new) => {
                        let mut data = into_fixed_float(self.data, size_orig, size_new);
                        let datatype = COMPLEX | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    FloatPackingOption::VariableLength(total_len) => {
                        let mut data = into_variable_float(self.data, size_orig, total_len);
                        let datatype = COMPLEX | VARIABLE_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                    FloatPackingOption::None => {
                        let mut data = self.data;
                        let datatype = COMPLEX | FIXED_LENGTH | self.shape.len() as u8;
                        write_footer(&mut data, datatype, self.shape);
                        data
                    }
                }
            }
            DataType::String => match inspect_string(&self.data, &self.shape) {
                StringPackingOption::Dictionary(dic) => {
                    let mut data = into_dictionary(self.data, dic);
                    let datatype = STRING | DICTIONARY | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    data
                }
                StringPackingOption::None => {
                    let mut data = self.data;
                    let datatype = STRING | JOINED | self.shape.len() as u8;
                    write_footer(&mut data, datatype, self.shape);
                    data
                }
            },
        }
    }
    fn pack_as_it_is(self) -> Vec<u8> {
        match self.datatype {
            DataType::UnsignedInteger => {
                let mut data = self.data;
                let datatype = UNSIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                write_footer(&mut data, datatype, self.shape);
                data
            }
            DataType::SignedInteger => {
                let mut data = self.data;
                let datatype = SIGNED_INTEGER | FIXED_LENGTH | self.shape.len() as u8;
                write_footer(&mut data, datatype, self.shape);
                data
            }
            DataType::Real => {
                let mut data = self.data;
                let datatype = REAL | FIXED_LENGTH | self.shape.len() as u8;
                write_footer(&mut data, datatype, self.shape);
                data
            }
            DataType::Complex => {
                let mut data = self.data;
                let datatype = COMPLEX | FIXED_LENGTH | self.shape.len() as u8;
                write_footer(&mut data, datatype, self.shape);
                data
            }
            DataType::String => {
                let mut data = self.data;
                let datatype = STRING | JOINED | self.shape.len() as u8;
                write_footer(&mut data, datatype, self.shape);
                data
            }
        }
    }
}

fn write_footer(data: &mut Vec<u8>, datatype: u8, shape: Vec<u64>) {
    let mut footer = [vec![datatype], varint_encode(shape)].concat();
    footer.reverse();
    data.append(&mut footer);
}
