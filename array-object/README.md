Array Object
============
[![Sponsors](https://img.shields.io/badge/offer-Coffee-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/array-object?style=flat-square)](https://crates.io/crates/array-object)
[![Crates.io](https://img.shields.io/crates/d/array-object?style=flat-square)](https://crates.io/crates/array-object)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/ArrayObject/blob/main/LICENSE.txt)

Self-describing binary format for arrays of integers, real numbers, complex numbers and strings, designed for object storage, database and single file.

`ArrayObject` is a part of [`dbgbb`](https://github.com/YShoji-HEP/dbgbb) project.

Highlights
----------
* The data is self-describing and can inflate itself into typed variables.
* No nested structures, no tuple, no dataset name, always a simple array of uniform data.
* Generic integer and float types absorb the difference of type sizes.
* Automatic compression using variable length integer/float and dictionary-coder for string.
* The data is stored in the minimal data size.
* Conversion from/into `ndarray` and `nalgebra` is supported.

Examples
--------
Encoding and decording:
```rust
use array_object::*;

fn main() {
    // Convert data into binary
    let original = vec![1u32, 2, 3, 4];
    let obj: ArrayObject = original.clone().try_into().unwrap();
    let packed = obj.pack(); // This converts the data into Vec<u8>.

    // Restore data
    let unpacked = ArrayObject::unpack(packed).unwrap();
    let inflated: Vec<u32> = unpacked.try_into().unwrap();
    assert_eq!(original, inflated);
}
```

One can also use the macros to write and read a file:
```rust
use array_object::*;

fn main() {
    // Save into a file
    let original = vec![1f64, 2.2, -1.1, 5.6];
    export_obj!("testdata.bin", original.clone()); // The type has to be known at this point.

    // Load from a file
    let restored: Vec<f64> = import_obj!("testdata.bin"); // The type annotation is required.
    assert_eq!(original, restored);
}
```

Crate Features
--------------
|Feature|Description|
|-|-|
|`allow_float_down_convert`|Allow implicit conversion such as from `f64` to `f32`.|
|`ndarray_15`|Enable ndarray support. The compatible version is 0.15.x.|
|`ndarray_16`|Enable ndarray support. The compatible version is 0.16.x.|
|`nalgebra`|Enable nalgebra support. Confirmed to work with version 0.33.0.|

Format
------
The data format is automatically selected to minimize the datasize.
### Integer
Integer is either unsigned or signed, which is determined when [`ArrayObject`] is constructed. The zigzag encoding is used for signed integers. When restored to a variable, data is automatically converted into the desired integer type if the ranges overlap.
#### Non-array
* **Short Integer** (5bit)<br />
The data is stored in the same byte as the footer. Thus the total data size is only one byte.

* **Variable Length** (8 x n bit)<br />
The integer is shortened to 8bit x (smallest number).

#### Array
* **Fixed Length** (8bit, 16bit, 32bit, 64bit, 128bit)<br />
Use the smallest possible size.
All the elements have the same size.
* **Variable Length** (8bit, 16bit, 32bit, 63bit, 64-128bit variable)<br />
The integer is shortened to the smallest possible size. Each four integers, one byte is added to indicate the size of each integer type. If the integer is longer than 63 bit, one byte is added to indicate how many bytes should be read additionally.

### Float (Real, Complex)<br />
Currently 32bit and 64bit floating numbers are supported.
#### Non-array
* **Fixed Length** (~~16bit~~, 32bit, 64bit, ~~128bit~~)<br />
Use the smallest possible size without loss of precision.
#### Array
* **Fixed Length** (~~16bit~~, 32bit, 64bit, ~~128bit~~)<br />
Use the smallest possible size without loss of precision. All the numbers have the same size.
* **Variable Length** (~~16bit~~, 32bit, 64bit, ~~128bit~~)<br />
The floating number is shortened to the smallest size. Fach four integers, one byte is added to indicate the size of each integer type.

### String
Only UTF-8 string is allowed, in particular, the non-UTF value of 0xFF is used internally and should be avoided.
#### Non-array
* **Single**<br />
Just a single Vec[u8] data
#### Array
* **Joined**<br />
The strings are joined with marker 0xFF, which never appears in UTF-8.
* **Dictionary**<br />
Create a dictionary of maximum 256 variants and the array is converted into an array of the references to the dictionary.

Q&A
--------------
#### When is it useful?
A simple case is to store a 2D array of numbers similar to a CSV file. Unlike a CSV file, `ArrayObject` can store arrays of more than two-dimensions, has a smaller filesize, and is fast to read and write.
Instead, it is not possible to append data to `ArrayObject` or to have different types of data in a single `ArrayObject`. The definition of `ArrayObject` is a chunk of data that is not separable. Hence there is no sense in appending data or having different types. In other words, `ArrayObject` is the data that we want to load into memory simultaneously, not line by line.
Another use is an object for object storage, which is actually what the `ArrayObject` is for. The object storage does the things that `ArrayObject` cannot: it allows you to append the data and store different types of data in a storage. This is why `ArrayObject` should not have these capabilities: having multiple options to do the same thing would complicate the system.
Instead, with a minimal footer, it does what the object storage cannot: type abstraction, forced type checking and type-dependent compression.

#### What is the difference from a raw binary file?
A raw binary file has no type checking system. Integers, for example, can be unsigned or signed, 32bit or 64bit, little endian or big endian, etc. Without type checking, one has to check that both reader and writer are using the same format.
When it comes to an array, it becomes more complicated: the raw memory order can be row major or column major depending on the programing language, and the real part and the imaginary part of complex numbers can be assigned to the most inner or the most outer index.
`ArrayObject` has a well defined spec and one does not need to worry about these things.

#### What is the difference from database like HDF5?
`ArrayObject` is a simple, compact and portable data format for a single array, but is not a database. Thus, it does not have complicated structure like dataset or group. Even so, `ArrayObject` is self-contained: it knows how to inflate itself and we do not need to feed any information to read the data.
Instead, `ArrayObject` does not have information of datasize, name, timestamp, permission, etc, which are supposed to be managed by object storage or filesystem. `ArrayObject` is rather closer to a CSV file: the program knows how to read it, but does not know what the data is about, when it is created, or what are the column and row, without accessing the metadata stored outside of the file.

#### What is the difference from serialization like Serde?
`ArrayObject` forbids nested structures: it can be an array of numbers or strings, but not of `ArrayObject`s themselves. It also cannot be a tuple. Such structures are supposed to be provided by the storage.
Serialization typically adds the datasize to each data to indicate the boundaries of the data. This information is not necessary for storing data in a file or a object storage because it is already managed by the storage. Instead, `ArrayObject` stores the information about the shape of the array.
A more technical difference is that `ArrayObject` adds a footer instead of a header, allowing metadata to be separated at no cost.

#### Why Complex Numbers?
The relation between complex numbers and real numbers is the same as that of real numbers and integers. A subset of complex numbers is real numbers and there is a well-defined map betwen them where they overlap. This makes a difference between a complex number and a vague array of length 2.
Practically, in loosely typed languages, the results of functions like sqrt or log yield either real numbers or complex numbers depending on the sign of the argument. It is thus useful to manifestly indicate complex numbers as a type and keep the same array shape.
In addition, it is somewhat cumbersome to convert an array of complex numbers into an array of real numbers. `ArrayObject` provides a handy export/import option for complex numbers.

#### Why is there no conversion from `Vec<Vec<_>>`?
`Vec<Vec<_>>` may contain vectors of different lengths.
In general, if you work with an fixed length array, it is much efficient to use a crate like ndarray or nalgebra.
For `ArrayObject`s having the same size, shape and type, `.try_concat()` method is available for `Vec<ArrayObject>`, which generates a one-dimensional higher array.