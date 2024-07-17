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
### output 
```rust
000 4A 4F 51 57 45 4A 52 45 4F 4A 4F 51 57 45 4A 52 45 4F 49 51  | JOQWEJREOJOQWEJREOIQ
020 49 57 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 51 49 57  | IWJEORIJOQWEJREOIQIW
040 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 0A 20 20 20 20  | JEORIJOQWEJREOI.....
060 51 49 57 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 51 49  | QIWJEORIJOQWEJREOIQI
080 4A 4F 51 57 45 4A 52 45 4F 4A 4F 51 57 45 4A 52 45 4F 49 51  | JOQWEJREOJOQWEJREOIQ
100 49 57 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 51 49 57  | IWJEORIJOQWEJREOIQIW
120 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 0A 20 20 20 20  | JEORIJOQWEJREOI.....
140 51 49 57 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 51 49  | QIWJEORIJOQWEJREOIQI
160 57 4A 45 4F 52 49 4A 76 51 49 57 4A 51 49 57 4A 45 4F 52 49  | WJEORIJvQIWJQIWJEORI
180 4A 4F 51 57 45 4A 52 45 4F 49 51 49 57 4A 45 4F 52 49 4A 76  | JOQWEJREOIQIWJEORIJv
200 76 4F 51 57 45 4A 52 45 4F 4A 4F 51 57 45 4A 52 45 4F 49 51  | vOQWEJREOJOQWEJREOIQ
220 49 57 4A 45 4F 52 49 4A 4F 51 57 45 4A 52 45 4F 49 51 49 57  | IWJEORIJOQWEJREOIQIW
240 4A 45 20 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  | JE.
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
### output
```rust
00 01 00 00 00 00 00 00 00 04 00 00 00 00 00 00 00 74 65 73 74  | ................test
```
## Serialized File
```rust
let buffer = fs::read("file path..").expect("read file error");
let mut m = Matrix::new(20, buffer.to_vec());
m.print_matrix();
```
### output 
```rust
00000 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00020 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72  | eqwerqweqwerqwe.qwer
00040 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65  | qweqwerqweqwerqweqwe
00060 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71  | rqweqwerqwe.qwerqweq
00080 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | werqweqwerqweqwerqwe
00100 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71  | qwerqwe.qwerqweqwerq
00120 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72  | weqwerqweqwerqweqwer
00140 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | qwe.qwerqweqwerqweqw
00160 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20  | erqweqwerqweqwerqwe.
00180 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00200 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | erqweqwerqweqwerqweq
00220 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77  | werqweqwerqwe.qwerqw
00240 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71  | eqwerqweqwerqweqwerq
00260 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65  | weqwerqwe.qwerqweqwe
00280 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | rqweqwerqweqwerqweqw
00300 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | erqwe.qwerqweqwerqwe
00320 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00340 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72  | e.qwerqweqwerqweqwer
00360 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77  | qweqwerqweqwerqwe.qw
00380 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | erqweqwerqweqwerqweq
00400 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77  | werqweqwerqwe.qwerqw
00420 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71  | eqwerqweqwerqweqwerq
00440 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65  | weqwerqwe.qwerqweqwe
00460 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | rqweqwerqweqwerqweqw
00480 65 72 71 77 65 20 61 73 64 66 71 77 65 72 71 77 65 71 77 65  | erqwe.asdfqwerqweqwe
00500 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | rqweqwerqweqwerqweqw
00520 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | erqwe.qwerqweqwerqwe
00540 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00560 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72  | e.qwerqweqwerqweqwer
00580 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77  | qweqwerqweqwerqwe.qw
00600 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | erqweqwerqweqwerqweq
00620 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77  | werqweqwerqwe.qwerqw
00640 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71  | eqwerqweqwerqweqwerq
00660 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65  | weqwerqwe.qwerqweqwe
00680 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | rqweqwerqweqwerqweqw
00700 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | erqwe.qwerqweqwerqwe
00720 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00740 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72  | e.qwerqweqwerqweqwer
00760 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 61 73  | qweqwerqweqwerqwe.as
00780 64 66 71 77 65 72 71 77 65 71 77 65 72 71 77 65 72 71 77 65  | dfqwerqweqwerqwerqwe
00800 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77  | qwerqweqwerqweqwerqw
00820 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72  | eqwerqwe.qwerqweqwer
00840 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65  | qweqwerqweqwerqweqwe
00860 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | rqwe.qwerqweqwerqweq
00880 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | werqweqwerqweqwerqwe
00900 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71  | .qwerqweqwerqweqwerq
00920 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65  | weqwerqweqwerqwe.qwe
00940 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77  | rqweqwerqweqwerqweqw
00960 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65  | erqweqwerqwe.qwerqwe
00980 71 77 65 72 71 77 65 71 77 65 72 71 77 65 72 71 77 65 71 77  | qwerqweqwerqwerqweqw
01000 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | erqweqwerqweqwerqweq
01020 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77  | werqwe.qwerqweqwerqw
01040 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71  | eqwerqweqwerqweqwerq
01060 77 65 20 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65  | we.qwerqweqwerqweqwe
01080 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71  | rqweqwerqweqwerqwe.q
01100 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65  | werqweqwerqweqwerqwe
01120 71 77 65 72 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71  | qwerqweqwerqwe.qwerq
01140 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72  | weqwerqweqwerqweqwer
01160 71 77 65 71 77 65 72 71 77 65 20 71 77 65 72 71 77 65 71 77  | qweqwerqwe.qwerqweqw
01180 65 72 71 77 65 71 77 65 72 71 77 65 71 77 65 72 71 77 65 71  | erqweqwerqweqwerqweq
..................
```