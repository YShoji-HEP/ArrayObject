use array_object::*;
use std::f64::consts::PI;

#[test]
fn single_real() {
    for i in -128..128 {
        let original = i as f64 * PI + 0.01;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 9);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: f64 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -128..128 {
        let original = i as f64 / 2f64.powi(3);
        let reference = &original;
        let obj: ArrayObject = reference.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 5);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: f64 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -128..128 {
        let original = i as f32 * 0.01;
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 5);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: f32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}

#[test]
fn array_real() {
    let original: Vec<_> = (-128..128).map(|i| i as f64 * PI + 0.01).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 * 8 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<f64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_fixed_real() {
    let original: Vec<_> = (-128..128).map(|i| i as f64 / 2f64.powi(3)).collect();
    let reference = &original;
    let obj: ArrayObject = reference.try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 * 4 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<f64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_variable_real() {
    let original: Vec<_> = (-128..128).map(|i| 0.2 * i as f64).collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 1908 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<f64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn zero_length() {
    let original: Vec<f64> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<f64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<f64> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let objs = vec![obj.clone(), obj.clone(), obj.clone()]
        .try_concat()
        .unwrap();
    let binary = objs.pack();
    assert_eq!(binary.len(), 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let adaptor::VecShape::<f64>(restored, shape) = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
    assert_eq!(shape, vec![3, 0])
}

#[test]
fn array() {
    let original = [1f64; 128];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: [f64; 128] = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}