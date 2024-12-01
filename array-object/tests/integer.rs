use array_object::*;

#[test]
fn short_integer() {
    for i in 0..32 {
        let original = i as u32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 1);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: u32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -16..16 {
        let original = i as i32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 1);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: i32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}

#[test]
fn single_variable_integer() {
    for i in 32..256 {
        let original = i as u32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 2);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: u32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in 256..256 * 256 {
        let original = i as u32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: u32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in 16..128 {
        let original = i as i32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 2);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: i32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -128..-16 {
        let original = i as i32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 2);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: i32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in 128..256 * 128 {
        let original = i as i32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: i32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -256 * 128..-128 {
        let original = i as i32;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: i32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}

#[test]
fn array_fixed_integer() {
    let original: Vec<_> = (-128..128).map(|i| i as i32).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<i32> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_variable_integer() {
    let original: Vec<_> = (0..256).map(|i| i as i32).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 448 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<i32> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<_> = (0..128).map(|i| u128::MAX >> i).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 1241 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<u128> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<_> = (0..128).map(|i| 1u128 << i).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 1241 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<u128> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn zero_length() {
    let original: Vec<i32> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<i32> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<u32> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let objs = vec![obj.clone(), obj.clone(), obj.clone()]
        .try_concat()
        .unwrap();
    let binary = objs.pack();
    assert_eq!(binary.len(), 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let adaptor::VecShape::<u32>(restored, shape) = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
    assert_eq!(shape, vec![3, 0])
}

#[test]
fn array() {
    let original = [2u32; 128];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: [u32; 128] = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn isize_usize() {
    let original: Vec<_> = (-128..128).map(|i| i as isize).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<isize> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<_> = (0..128).map(|i| i as usize).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<usize> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

