use std::fs;
use std::io::{ Error as IOError };
use std::path::{ PathBuf };
use std::error::Error;

pub fn read_file( path: &PathBuf ) -> Result<(), IOError> {
    Ok( () )
}

#[cfg(test)]
mod test;
