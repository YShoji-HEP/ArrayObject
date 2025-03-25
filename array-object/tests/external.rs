#[cfg(any(feature = "ndarray_15", feature = "ndarray_16"))]
mod test_ndarray {
    use array_object::*;
    use ndarray::{Array1, Array2};
    #[cfg(feature = "ndarray_15")]
    use ndarray_15 as ndarray;
    #[cfg(feature = "ndarray_16")]
    use ndarray_16 as ndarray;
    #[test]
    fn ndarray_integer() {
        let v: Vec<_> = (-128..128).map(|i| i as i32).collect();
        let original = Array2::from_shape_vec((16, 16), v).unwrap();
        let obj: ArrayObject = original.clone().try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 256 + 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: Array2<i32> = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
    #[test]
    fn ndarray_float() {
        let original: Array1<_> = (-128..128).map(|i| i as f64).collect();
        let view = original.view();
        let obj: ArrayObject = view.try_into().unwrap();
        let binary = obj.pack();
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: Array1<f64> = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}

#[cfg(feature = "nalgebra")]
mod test_nalgebra {
    use array_object::*;
    use nalgebra::DMatrix;
    #[test]
    fn nalgebra_integer() {
        let v: Vec<_> = (-128..128).map(|i| i as i32).collect();
        let original = DMatrix::from_vec(16, 16, v);
        let reference = &original;
        let obj: ArrayObject = reference.try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 256 + 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: DMatrix<i32> = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}
