use std::fs;

use bincode::serialize;
use hex_matrix::matrix::{Line, Matrix};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    a: u64,
    b: String,
}

fn main() {
    // serialized string
    println!("-------------------------------- serialized string --------------------------------");
    let arr = "JOQWEJREOJOQWEJREOIQIWJEORIJOQWEJREOIQIWJEORIJOQWEJREOI
    QIWJEORIJOQWEJREOIQIJOQWEJREOJOQWEJREOIQIWJEORIJOQWEJREOIQIWJEORIJOQWEJREOI
    QIWJEORIJOQWEJREOIQIWJEORIJvQIWJQIWJEORIJOQWEJREOIQIWJEORIJvvOQWEJREOJOQWEJREOIQIWJEORIJOQWEJREOIQIWJE ".as_bytes();
    let mut m = Matrix::new(20, arr.to_vec());
    m.print_matrix();
    // serialized struct
    println!("-------------------------------- serialized struct --------------------------------");
    let t = Test {
        a: 1,
        b: String::from("test"),
    };
    let encoded: Vec<u8> = serialize(&t).unwrap();
    let mut m = Matrix::new(20, encoded.to_vec());
    m.print_matrix();
    // serialized file
    println!("-------------------------------- serialized file --------------------------------");
    let buffer = fs::read("/resource/TEST_1").expect("read file error");
    let mut m = Matrix::new(20, buffer.to_vec());
    m.print_matrix();
}
