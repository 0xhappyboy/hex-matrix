# hex-matrix
build hex matrix using byte array
# Examples
## Instantiate
```rust
let b_arr = vec![];
let mut m = Matrix::new(20, arr.to_vec());
```
## Serialized String
```rust
let arr = "".as_bytes();
let mut m = Matrix::new(20, arr.to_vec());
m.print_matrix();
```
## Serialized Struct
```rust
let t = Test {
    a: 1,
    b: String::from("test"),
};
let encoded: Vec<u8> = serialize(&t).unwrap();
let mut m = Matrix::new(20, encoded.to_vec());
m.print_matrix();
```
## Serialized File
```rust
let buffer = fs::read("file path..").expect("read file error");
let mut m = Matrix::new(20, buffer.to_vec());
m.print_matrix();
```