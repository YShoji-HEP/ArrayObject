use array_object::*;

#[test]
fn single_string() {
    let original = "test".to_string();
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), original.len() + 1);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: String = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_joined_string() {
    let original = vec!["testA".to_string(), "testB".to_string()];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 2 * "testA".to_string().len() + 1 + 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<String> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn array_dictionary_string() {
    let original = vec![
        "testA".to_string(),
        "testB".to_string(),
        "testA".to_string(),
        "testB".to_string(),
    ];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 17 + 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<String> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}

#[test]
fn zero_length() {
    let original: Vec<String> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    assert_eq!(binary.len(), 2);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: Vec<String> = unpacked.try_into().unwrap();
    assert_eq!(original, restored);

    let original: Vec<String> = vec![];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let objs = vec![obj.clone(), obj.clone(), obj.clone()]
        .try_concat()
        .unwrap();
    let binary = objs.pack();
    assert_eq!(binary.len(), 3);
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let adaptor::VecShape::<String>(restored, shape) = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
    assert_eq!(shape, vec![3, 0])
}

#[test]
fn array() {
    let original = ["test".to_string(), "test".to_string(), "test".to_string()];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let binary = obj.pack();
    let unpacked = ArrayObject::unpack(binary).unwrap();
    let restored: [String; 3] = unpacked.try_into().unwrap();
    assert_eq!(original, restored);
}
