pub fn varint_encode(numbers: Vec<u64>) -> Vec<u8> {
    let mut res = vec![];
    for mut number in numbers.iter().cloned() {
        loop {
            let next = (number as u8) & 0b0111_1111u8;
            number >>= 7;
            if number == 0 {
                res.push(next);
                break;
            } else {
                res.push(next | 0b1000_0000u8);
            }
        }
    }
    res
}

pub fn varint_decode<'a>(
    varint: impl Iterator<Item = &'a u8>,
    max_len: usize,
) -> (Vec<u64>, usize) {
    let mut res = vec![];
    let mut temp = 0;
    let mut i = 0;
    let mut len = 0;
    if max_len == 0 {
        return (res, 0);
    }
    for v in varint {
        temp += ((v & 0b0111_1111u8) as u64) << (7 * i);
        i += 1;
        if v & 0b1000_0000u8 == 0 {
            len += i;
            res.push(temp);
            if res.len() < max_len {
                i = 0;
                temp = 0;
            } else {
                break;
            }
        }
    }
    (res, len)
}
