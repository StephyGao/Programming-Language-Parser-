use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;


pub struct CharStream {
    contents: Vec<char>,
    curr_pos: usize,
    end_pos: usize
}

impl CharStream {

    pub fn new(f: &str) -> CharStream {
        let mut file = File::open(f)
            .expect("Unable to open file.");
        let mut contents = String::new();
        let end_pos = file.read_to_string(&mut contents)
            .expect("Unable to read file.");
        CharStream {
            contents: contents.chars().collect(),
            curr_pos: 0,
            end_pos
        }
    }

    pub fn from_str(s: &str) -> CharStream {
        CharStream {
            contents: s.chars().collect(),
            curr_pos: 0,
            end_pos: s.chars().count()
        }
    }
    
    // Returns true if more characters are available, false otherwise.
    pub fn more_available(&self) -> bool {
        if self.curr_pos < self.end_pos {
            true
        } else {
            false
        }
    }

    // Returns the next character without consuming it.
    // Returns None if no more characters are available. 
    pub fn peek_next_char(&self) -> Option<char> {
        self.peek_ahead_char(0)
    }

    // Returns the kth character ahead in the stream without consuming it.
    // peek_ahead_char(0) returns the same character as peek_next_char().
    // Returns None if no more characters are available at the position.
    // The input k cannot be negative.
    pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
        let mut ret = None;
        let k_usize = k as usize;
        if self.curr_pos + k_usize < self.end_pos {
            ret = Some(self.contents[self.curr_pos + k_usize]);
        }
        ret
    }

    // Returns the next character and consumes it.
    // Returns None if no more characters are available.
    pub fn get_next_char(&mut self) -> Option<char> {
        let mut ret = None;
        if self.curr_pos < self.end_pos {
            ret = Some(self.contents[self.curr_pos]);
            self.curr_pos += 1;
        }
        ret
    }
}

