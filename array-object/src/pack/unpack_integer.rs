
pub fn from_variable_integer(packed: Vec<u8>) -> Vec<u8> {
    let mut data = vec![];
    let mut pos_header = 0;
    loop {
        if pos_header >= packed.len() {
            break;
        }
        let mut header = packed[pos_header];
        let mut ln2_size = vec![];
        for _ in 0..4 {
            ln2_size.push((header & 0b1100_0000u8) >> 6);
            header <<= 2;
        }
        let mut pos = pos_header + 1;
        for l in ln2_size {
            match l {
                0 => {
                    data.push(vec![packed[pos]]);
                    pos += 1;
                }
                1 => {
                    data.push(vec![packed[pos], packed[pos + 1]]);
                    pos += 2;
                }
                2 => {
                    let mut temp = vec![];
                    for i in 0..4 {
                        temp.push(packed[pos + i]);
                    }
                    data.push(temp);
                    pos += 4;
                }
                3 => {
                    let mut temp = vec![];
                    if packed[pos + 7] & 0b1000_0000u8 == 0 {
                        for i in 0..8 {
                            temp.push(packed[pos + i]);
                        }
                        pos += 8;
                    } else {
                        let len = (packed[pos + 7] * 0b000_0001) as usize;
                        for i in 0..7 {
                            temp.push(packed[pos + i]);
                        }
                        for i in 8..len + 1 {
                            temp.push(packed[pos + i]);
                        }
                        pos += len + 1;
                    }
                    data.push(temp);
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
    if let Some(max_bytes) = data.iter().map(|x| x.len()).max() {
        for x in &mut data {
            for _ in 0..(max_bytes - x.len()) {
                x.push(0);
            }
        }
        data.concat()
    } else {
        vec![]
    }
}