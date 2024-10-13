/// Pair(re, im) for a single complex.
pub struct Pair<T>(pub T, pub T);
/// VecShape(elements in row major order, shape of array) for any type T.
pub struct VecShape<T>(pub Vec<T>, pub Vec<u64>);
/// VecVec(re vector, im vector) for complex vector.
pub struct VecVec<T>(pub Vec<T>, pub Vec<T>);
/// VecVecShape(real elements in row major order, imaginary elements in row major order, shape of array) for complex array.
pub struct VecVecShape<T>(pub Vec<T>, pub Vec<T>, pub Vec<u64>);