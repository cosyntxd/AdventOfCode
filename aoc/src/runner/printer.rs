const COLUMN_WIDTH: usize = 20;
use std::io::{self, Write};


pub struct ColumnPrinter {
    column_height: usize,
    column_width: usize,
    curr_column: usize,
    curr_max_line: usize,
}

impl ColumnPrinter {
    pub fn new(column_height: usize, column_width: usize) -> Self {
        Self {
            column_height: 0,
            column_width: 0,
            curr_column: 0,
            curr_max_line: 0,
        }
    }

    pub fn new_column(&mut self) {
    }


    pub fn print(&mut self, s: &str) {
        
    }
}