use crate::misc::Product;
use std::collections::HashMap;

#[derive(Debug)]
pub enum StringPackingOption {
    None,
    Dictionary(Vec<Vec<u8>>),
}

pub fn inspect_string(data: &Vec<u8>, shape: &Vec<u64>) -> StringPackingOption {
    let total_len = shape.product();
    if total_len == 1 {
        return StringPackingOption::None;
    }
    let mut dictionary: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut prev_pos = 0;
    while let Some(p) = data.iter().skip(prev_pos).position(|&x| x == 255) {
        dictionary
            .entry(data[prev_pos..prev_pos + p].to_vec())
            .and_modify(|i| *i += 1)
            .or_default();
        prev_pos += p + 1;
    }
    {
        dictionary
            .entry(data[prev_pos..].to_vec())
            .and_modify(|i| *i += 1)
            .or_default();
    }
    let n_var = dictionary.len() as u64;
    let mut size_key: u64 = 0;
    for key in dictionary.keys() {
        size_key += key.len() as u64;
    }
    let len_orig = data.len() as u64;
    let len_dictionary = total_len as u64 + size_key + n_var + 1;
    if len_dictionary < len_orig && n_var < 256 {
        StringPackingOption::Dictionary(dictionary.into_keys().collect())
    } else {
        StringPackingOption::None
    }
}

pub fn into_dictionary(mut data_orig: Vec<u8>, dictionary: Vec<Vec<u8>>) -> Vec<u8> {
    let mut data = vec![];
    while let Some(p) = data_orig.iter().position(|&x| x == 255) {
        let mut s = data_orig.split_off(p + 1);
        std::mem::swap(&mut s, &mut data_orig);
        s.pop();
        data.push(dictionary.iter().position(|x| *x == s).unwrap() as u8);
    }
    data.push(dictionary.iter().position(|x| *x == data_orig).unwrap() as u8);
    let mut header = vec![dictionary.len() as u8];
    for mut d in dictionary {
        header.append(&mut d);
        header.push(255);
    }
    [header, data].concat()
}
