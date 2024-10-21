use crate::misc::Product;
use crate::storage::TYPE_MASK;

#[derive(Debug)]
pub enum IntegerPackingOption {
    None,
    FixedLength(usize),
    VariableLength(u64),
    Short,
    ShortVariable,
}

pub fn inspect_integer(data: &Vec<u8>, size_orig: usize, shape: &Vec<u64>) -> IntegerPackingOption {
    if shape.is_empty()
        && data.len() > 0
        && data[0] & TYPE_MASK == 0
        && data.iter().skip(1).all(|b| *b == 0)
    {
        return IntegerPackingOption::Short;
    }
    if shape.product() == 1 && data[data.len() - 1] == 0 {
        return IntegerPackingOption::ShortVariable;
    }
    let mut count: Vec<u64> = vec![0; 5];
    let mut additional: u64 = 0;
    for x in data.chunks(size_orig) {
        let pos = x.iter().rev().position(|i| *i > 0).unwrap_or(size_orig - 1);
        let min_size = size_orig - pos;
        match min_size {
            1 => {
                count[0] += 1;
            }
            2 => {
                count[1] += 1;
            }
            3..=4 => {
                count[2] += 1;
            }
            5..=8 => {
                count[3] += 1;
                if x[7] & 0b1000_0000u8 != 0 {
                    additional += 1;
                }
            }
            _ => {
                count[4] += 1;
                additional += min_size as u64 + 1;
            }
        }
    }
    let len_orig = data.len() as u64;
    let n_elem = len_orig / size_orig as u64;
    let pos = count.iter().rev().position(|i| *i > 0).unwrap_or(4);
    let size_fixed = 2u64.pow(4 - pos as u32);
    let len_fixed = size_fixed * n_elem;
    let len_variable =
        count[0] + count[1] * 2 + count[2] * 4 + count[3] * 8 + additional + (n_elem - 1) / 4 + 1;

    if len_fixed > len_variable {
        if len_variable < data.len() as u64 {
            IntegerPackingOption::VariableLength(len_variable)
        } else {
            IntegerPackingOption::None
        }
    } else {
        if len_fixed < len_orig {
            IntegerPackingOption::FixedLength(size_fixed as usize)
        } else {
            IntegerPackingOption::None
        }
    }
}

pub fn into_fixed_integer(data_orig: Vec<u8>, size_orig: usize, size_new: usize) -> Vec<u8> {
    let step = size_orig / size_new;
    data_orig
        .chunks(size_new)
        .step_by(step)
        .map(|x| x.to_vec())
        .flatten()
        .collect()
}

pub fn into_variable_integer(data_orig: Vec<u8>, size_orig: usize, total_len: u64) -> Vec<u8> {
    let mut data = Vec::<u8>::with_capacity(total_len.try_into().unwrap());
    for x in data_orig.chunks(4 * size_orig) {
        let mut size: Vec<u8> = vec![];
        let mut temp = vec![];
        for x in x.chunks(size_orig) {
            let pos = x.iter().rev().position(|i| *i > 0).unwrap_or(size_orig - 1);
            let min_size = size_orig - pos;
            match min_size {
                1 => {
                    temp.push(x[0]);
                    size.push(0);
                }
                2 => {
                    temp.push(x[0]);
                    temp.push(x[1]);
                    size.push(1);
                }
                3..=4 => {
                    for i in 0..4 {
                        temp.push(x[i]);
                    }
                    size.push(2);
                }
                5..=8 => {
                    for i in 0..7 {
                        temp.push(x[i]);
                    }
                    if x[7] & 0b1000_0000u8 == 0 {
                        temp.push(x[7]);
                    } else {
                        temp.push(8u8 | 0b1000_0000u8);
                        temp.push(x[7]);
                    }
                    size.push(3);
                }
                _ => {
                    for i in 0..7 {
                        temp.push(x[i]);
                    }
                    temp.push(min_size as u8 | 0b1000_0000u8);
                    for i in 7..min_size {
                        temp.push(x[i]);
                    }
                    size.push(3);
                }
            }
        }
        let mut header = 0u8;
        for l in &size {
            header <<= 2;
            header |= l;
        }
        header <<= 2 * (4 - size.len());
        data.push(header);
        data.append(&mut temp);
    }
    data
}

pub fn into_short_variable_integer(mut data_orig: Vec<u8>) -> Vec<u8> {
    while data_orig.len() > 0 && *data_orig.last().unwrap() == 0 {
        data_orig.pop().unwrap();
    }
    data_orig
}
