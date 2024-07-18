//! matrix module

use std::{
    default,
    ops::{Add, Div},
};

#[derive(Default, Clone)]
pub struct Line {
    no: String,
    hex: String,
    info: String,
}

impl Line {
    pub fn no(&self) -> &str {
        &self.no
    }
    pub fn hex(&self) -> &str {
        &self.hex
    }
    pub fn info(&self) -> &str {
        &self.info
    }
    pub fn push_no(&mut self, no: String) {
        self.no.push_str(&no);
    }
    pub fn push_hex(&mut self, hex: String) {
        self.hex.push_str(&hex);
    }
    pub fn push_info(&mut self, info: String) {
        self.info.push_str(&info);
    }
    pub fn set_no(&mut self, no: String) {
        self.no = no;
    }
    pub fn set_hex(&mut self, hex: String) {
        self.hex = hex;
    }
    pub fn set_info(&mut self, info: String) {
        self.info = info;
    }
    pub fn to_string(&self) -> String {
        format!("{} {} | {}", self.no, self.hex, self.info)
    }
}

pub struct Matrix {
    width: usize,
    b_arr: Vec<u8>,
    lines: Vec<Line>,
}

/// Hexadecimal matrix structure
///
/// # Examples
///
/// ```rust
/// let arr = "".as_bytes();
/// let mut m = Matrix::new(20, arr.to_vec());
/// ```
impl Matrix {
    pub fn new(w: usize, b: Vec<u8>) -> Self {
        Self {
            width: w,
            b_arr: b.clone(),
            lines: Self::to_matrix(w, b.clone()),
        }
    }

    /// print matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arr = "".as_bytes();
    /// let mut m = Matrix::new(20, arr.to_vec());
    /// m.print_matrix();
    /// ```
    pub fn print_matrix(&mut self) {
        for (i, v) in self.lines.iter().enumerate() {
            println!("{}", v.to_string());
        }
    }

    /// reset matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arr = "".as_bytes();
    /// let mut m = Matrix::new(20, arr.to_vec());
    ///
    /// let arr2 = "".as_bytes();
    /// m.reset(arr2);
    /// ```
    pub fn reset(&mut self, b: &[u8]) {
        self.b_arr = b.to_vec();
        Self::to_matrix(self.width, b.to_vec());
    }

    /// reset matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arr = "".as_bytes();
    /// let mut m = Matrix::new(20, arr.to_vec());
    ///
    /// let line = m.get_line_by_index(index);
    /// ```
    pub fn get_line_by_index(&mut self, index: usize) -> &mut Line {
        self.lines.get_mut(index).unwrap()
    }

    /// reset matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arr = "".as_bytes();
    /// let mut m = Matrix::new(20, arr.to_vec());
    ///
    /// println("{}", m.line_count());
    /// ```
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// get matrix line
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arr = "".as_bytes();
    /// let mut m = Matrix::new(20, arr.to_vec());
    ///
    /// let lines : Vec<Line> = m.lines();
    /// for (i,v) in lines.iter().enumerate()
    /// {
    ///   // ....
    /// }
    /// ```
    pub fn lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    fn to_matrix(width: usize, b_arr: Vec<u8>) -> Vec<Line> {
        let len = b_arr.len();
        let mut lines: Vec<Line> = vec![];
        let mut line = Line::default();
        // max line count
        let mut max_line_count = len.div(width);
        max_line_count = if max_line_count % width > 0 {
            max_line_count.add(1)
        } else {
            max_line_count
        };
        // max line number
        let mut max_line_number = String::from(format!("{}", (max_line_count * width))).len();
        // current line number
        let mut current_line_number: usize = 0;
        for (i, b) in b_arr.iter().enumerate() {
            // line number
            let line_number: usize = current_line_number * width;
            line.push_hex(format!("{:<02X} ", b));
            if i % width == width - 1 || i == len - 1 {
                for _j in 0..width - 1 - (i % (width)) {
                    line.push_hex("00 ".to_string());
                }
                for j in i - i % width..i + 1 {
                    if b_arr[j].is_ascii_alphabetic() {
                        line.push_info(format!("{}", b_arr[j] as char));
                    } else {
                        line.push_info(".".to_string());
                    }
                }
                let mut fill_zero = String::from("");
                for i in 0..(max_line_number - line_number.to_string().len()) {
                    fill_zero.push_str("0");
                }
                line.set_no(format!("{}{}", fill_zero, line_number.to_string()));
                lines.push(line.clone());
                line = Line::default();
                current_line_number += 1;
            }
        }
        lines
    }
}
