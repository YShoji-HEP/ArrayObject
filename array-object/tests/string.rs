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
