pub fn from_dictionary(packed: Vec<u8>) -> Vec<u8> {
    let dic_len = packed[0];
    let mut prev_pos = 1;
    let mut dictionary = vec![];
    for _ in 0..dic_len {
        let p = packed
            .iter()
            .skip(prev_pos)
            .position(|&x| x == 255)
            .unwrap();
        dictionary.push(packed[prev_pos..prev_pos + p].to_vec());
        prev_pos += p + 1;
    }
    let mut data = vec![];
    for key in packed.iter().skip(prev_pos) {
        data.push(dictionary[*key as usize].clone());
        data.push(vec![255]);
    }
    data.pop();
    data.concat()
}
