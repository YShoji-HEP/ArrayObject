use array_object::*;
use num_complex::{Complex32, Complex64};
use std::f64::consts::PI;

#[test]
fn single_complex() {
    for i in -128..128 {
        let original = Complex64::new(i as f64 * PI + 0.01, i as f64 * PI - 0.01);
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 17);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: Complex64 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -128..128 {
        let original = Complex64::new(i as f64 / 2f64.powi(3), i as f64 / 2f64.powi(4));
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 9);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: Complex64 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    for i in -128..128 {
        let original = Complex32::new(i as f32 * 0.01, i as f32 * 0.02);
        let obj: ArrayObject = original.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 9);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: Complex32 = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}

#[test]
fn array_complex() {
    let original: Vec<_> = (-128..128)
        .map(|i| Complex64::new(i as f64 * PI + 0.01, i as f64 * PI - 0.01))
        .collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 * 8 * 2 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<Complex64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<_> = (-128..128)
        .map(|i| Complex32::new(i as f32 * 3.14 + 0.01, i as f32 * 3.14 - 0.01))
        .collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 * 4 * 2 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<Complex32> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_fixed_complex() {
    let original: Vec<_> = (-128..128)
        .map(|i| Complex64::new(i as f64 / 2f64.powi(3), i as f64 / 2f64.powi(4)))
        .collect();
    let reference = &original;
    let obj: ArrayObject = reference.try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 256 * 4 * 2 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<Complex64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_variable_complex() {
    let original: Vec<_> = (-128..128)
        .map(|i| Complex64::new(0.2 * i as f64, 0.2 * i as f64))
        .collect();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 1908 * 2 + 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<Complex64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn zero_length() {
    let original: Vec<Complex64> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<Complex64> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<Complex64> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let objs = vec![obj.clone(), obj.clone(), obj.clone()]
        .try_concat()
        .unwrap();
    let binary = objs.pack();
    assert_eq!(binary.len(), 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let adaptor::VecShape::<Complex64>(restored, shape) = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
    assert_eq!(shape, vec![3, 0])
}

#[test]
fn array() {
    let original = [Complex64::new(1., 2.); 128];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: [Complex64; 128] = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}
