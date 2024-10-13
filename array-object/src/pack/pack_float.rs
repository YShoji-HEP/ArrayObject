#[derive(Debug)]
pub enum FloatPackingOption {
    None,
    FixedLength(usize),
    VariableLength(u64),
}

fn lossless(x: &[u8], _size_orig: usize, _size_new: usize) -> Option<Vec<u8>> {
    if x[0] != 0 {
        return None;
    }
    let x = f64::from_le_bytes(x.try_into().unwrap());
    let y = x as f32;
    if (y as f64).to_bits() == x.to_bits() {
        Some(y.to_le_bytes().to_vec())
    } else {
        None
    }
}

pub fn inspect_float(data: &Vec<u8>, size_orig: usize) -> FloatPackingOption {
    if size_orig == 4 {
        return FloatPackingOption::None;
    }
    let mut count: Vec<u64> = vec![0; 4];
    for x in data.chunks(size_orig) {
        if let Some(_) = lossless(x, size_orig, 4) {
            count[1] += 1;
        } else {
            count[2] += 1;
        }
    }
    let len_orig = data.len() as u64;
    let n_elem = len_orig / size_orig as u64;
    let pos = count.iter().rev().position(|i| *i > 0).unwrap_or(1);
    let size_fixed = 2u64.pow(4 - pos as u32);
    let len_fixed = size_fixed * n_elem;
    let len_variable = count[1] * 4 + count[2] * 8 + (n_elem - 1) / 4 + 1;

    if len_fixed > len_variable {
        if len_variable < data.len() as u64 {
            FloatPackingOption::VariableLength(len_variable)
        } else {
            FloatPackingOption::None
        }
    } else {
        if len_fixed < len_orig {
            FloatPackingOption::FixedLength(size_fixed as usize)
        } else {
            FloatPackingOption::None
        }
    }
}

pub fn into_fixed_float(data_orig: Vec<u8>, size_orig: usize, size_new: usize) -> Vec<u8> {
    match size_orig {
        8 => match size_new {
            4 => data_orig
                .chunks(size_orig)
                .map(|x| {
                    (f64::from_le_bytes(x.try_into().unwrap()) as f32)
                        .to_le_bytes()
                        .to_vec()
                })
                .flatten()
                .collect(),
            8 => {
                panic!();
            }
            _ => {
                panic!();
            }
        },
        _ => {
            panic!();
        }
    }
}

pub fn into_variable_float(data_orig: Vec<u8>, size_orig: usize, len: u64) -> Vec<u8> {
    let mut data = Vec::<u8>::with_capacity(len.try_into().unwrap());
    for x in data_orig.chunks(4 * size_orig) {
        let mut size: Vec<u8> = vec![];
        let mut temp = vec![];
        for x in x.chunks(size_orig) {
            if let Some(mut y) = lossless(x, size_orig, 4) {
                temp.append(&mut y);
                size.push(1);
            } else {
                temp.append(&mut x.to_vec());
                size.push(2);
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
