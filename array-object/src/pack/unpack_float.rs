pub fn from_variable_float(packed: Vec<u8>) -> Vec<u8> {
    let mut data = vec![];
    let mut pos_header = 0;
    while pos_header < packed.len() {
        let mut header = packed[pos_header];
        let mut ln2_size = vec![];
        for _ in 0..4 {
            ln2_size.push((header & 0b1100_0000u8) >> 6);
            header <<= 2;
        }
        let mut pos = pos_header + 1;
        for l in ln2_size {
            match l {
                1 => {
                    let mut temp = vec![];
                    for i in 0..4 {
                        temp.push(packed[pos + i]);
                    }
                    data.push(temp);
                    pos += 4;
                }
                2 => {
                    let mut temp = vec![];
                    for i in 0..8 {
                        temp.push(packed[pos + i]);
                    }
                    data.push(temp);
                    pos += 8;
                }
                _ => {
                    panic!();
                }
            }
            if pos >= packed.len() {
                break;
            }
        }
        pos_header = pos;
    }
    if let Some(max_size) = data.iter().map(|x| x.len()).max() {
        for x in &mut data {
            if max_size > x.len() {
                *x = (f32::from_le_bytes(x.as_slice().try_into().unwrap()) as f64)
                    .to_le_bytes()
                    .to_vec();
            }
        }
        data.concat()
    } else {
        vec![]
    }
}
