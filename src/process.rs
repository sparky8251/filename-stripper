use std::{fs, io, path::PathBuf};

use crate::utils::{get_filenames, get_files};

pub fn process(path: PathBuf, number: usize) -> Result<(), io::Error> {
    match get_files(&path) {
        Ok(files) => {
            for file in files {
                let original_file = file.path();
                let renamed_file = get_filenames(&file, number, &path);

                if let Err(e) = fs::rename(&original_file, &renamed_file) {
                    eprintln!(
                        "Unable to rename original file {:?} to {:?} due to error {}",
                        original_file, renamed_file, e
                    )
                }
            }
        }
        Err(e) => return Err(e),
    };

    Ok(())
}
