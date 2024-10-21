#[cfg(feature = "ndarray")]
mod test_ndarray {
    use array_object::*;
    use ndarray::Array2;
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
}

#[cfg(feature = "nalgebra")]
mod test_ndarray {
    use array_object::*;
    use nalgebra::DMatrix;
    #[test]
    fn ndarray_integer() {
        let v: Vec<_> = (-128..128).map(|i| i as i32).collect();
        let original = DMatrix::from_vec(16,16,v);
        let obj: ArrayObject = original.clone().try_into().unwrap();
        let binary = obj.pack();
        assert_eq!(binary.len(), 256 + 3);
        let unpacked = ArrayObject::unpack(binary).unwrap();
        let restored: DMatrix<i32> = unpacked.try_into().unwrap();
        assert_eq!(original, restored);
    }
}