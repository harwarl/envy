use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines, Read},
};

use crate::errors::*;

pub struct Iter<R> {
    lines: Lines<BufReader<R>>,
    substitution_data: HashMap<String, Option<String>>,
}

impl<R: Read> Iter<R> {
    pub fn new(reader: R) -> Self {
        Iter {
            lines: BufReader::new(reader).lines(),
            substitution_data: HashMap::new(),
        }
    }

    pub fn load(&self) -> Self {
        todo!()
    }
}

impl<R: Read> Iterator for Iter<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(Error::Io(err))),
                None => return None,
            };

            // TODO: fix parse the parse lines
        }
    }
}
