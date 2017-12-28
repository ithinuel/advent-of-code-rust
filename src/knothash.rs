use std::fmt::{Debug, Display, Formatter, Result};

pub struct KnotHash {
    lengths: Vec<usize>,
    array: Vec<u8>,
    skip_size: usize,
    cur_pos: usize
}

impl KnotHash {
    pub fn new(bytes: &[usize], len: usize) -> KnotHash {
        KnotHash {
            lengths: bytes.to_vec(),
            array: (0..len).map(|n| n as u8).collect(),
            skip_size: 0,
            cur_pos: 0
        }
    }

    fn swap_slice_at(&self, at: usize) -> Vec<u8> {
        let (start, end) = self.array.split_at(at);
        let mut v = Vec::with_capacity(self.array.len());
        v.extend_from_slice(end);
        v.extend_from_slice(start);
        v
    }

    pub fn round(&mut self) {
        for length in &self.lengths {
            self.array[0..*length].reverse();
            let jmp = (length + self.skip_size) % self.array.len();
            self.cur_pos = (self.cur_pos + jmp) % self.array.len();
            self.array = self.swap_slice_at(jmp);
            self.skip_size += 1;
        }
    }
    
    pub fn get_sparse_hash(&self) -> Vec<u8> {
        self.swap_slice_at(self.array.len() - self.cur_pos)
    }

    pub fn get_dense_hash_as_vec(&self) -> Vec<u8> {
        self.get_sparse_hash()
            .chunks(16)
            .map(|c| c.iter().fold(0, |x, v| x^v))
            .collect()
    }

    pub fn get_dense_hash_as_string(&self) -> String {
        self.get_dense_hash_as_vec()
            .iter()
            .map(|v| format!("{:02x}", v))
            .collect()
    }
}

impl Debug for KnotHash {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.get_sparse_hash())
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.get_dense_hash_as_string())
    }
}

