use std::{fs, path::PathBuf, process::ExitCode};

use crate::utils::{get_dir_contents, get_filenames};

pub fn process(path: PathBuf, number: usize) -> ExitCode {
    match get_dir_contents(&path) {
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
        Err(e) => return e,
    };

    ExitCode::SUCCESS
}
