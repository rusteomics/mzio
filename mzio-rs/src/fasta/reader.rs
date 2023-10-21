// std imports
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// 3rd party imports
use anyhow::Result;

// internal imports
use crate::fasta::entry::Entry;
use crate::fasta::headers::Header;


/// Reader for common FASTA files as distributed by e.g. UniProt (https://uniprot.org)
pub struct Reader<T> where T: Header {
    internal_reader: BufReader<File>,
    is_eof: bool,
    header: String,
    sequence: String,
    _header_phantom: std::marker::PhantomData<T>
}

impl<T> Reader<T> where T: Header {
    /// Creates a new Reader
    /// # Arguments
    ///
    /// * `fasta_file_path` - Path to FASTA file
    /// 
    pub fn new(fasta_file_path: &Path, buffer_size: usize) -> Result<Self> {
        let fasta_file: File = File::open(fasta_file_path)?;
        Ok(Self {
            internal_reader: BufReader::with_capacity(buffer_size, fasta_file),
            is_eof: false,
            header: String::new(),
            sequence: String::new(),
            _header_phantom: std::marker::PhantomData
        })
    }

    pub fn create_entry(header: &str, sequence: &str) -> Option<Entry<T>> {
        Some(
            Entry::new(
                T::new(header),
                sequence.to_owned()
            )
        )
    }
}


impl<T> Iterator for Reader<T> where T: Header {
    type Item = Entry<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }
        self.sequence = String::new(); // Reset sequence, new header is still present from last iteration.
        loop {
            let mut line = String::new();
            if let Ok(num_bytes) = self.internal_reader.read_line(&mut line) {
                if num_bytes == 0 {
                    self.is_eof = true;
                    return Reader::create_entry(&self.header, &self.sequence);
                }
                line = line.as_mut_str().trim().to_string();
                if !line.starts_with(">") && num_bytes > 0 {
                    self.sequence.push_str(&line)
                } else {
                    if self.header.len() > 0 {
                        let entry = Reader::create_entry(&self.header, &self.sequence);
                        self.header = line; // safe newly read header
                        return entry;
                    } else  {
                        self.header = line; 
                    }
                }
            }
        }
        
    }
}
