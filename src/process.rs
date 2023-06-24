use std::{
    path::PathBuf,
    process::{Command, ExitCode},
};

use crate::utils::{get_dir_contents, get_filenames};

pub fn process(path: PathBuf, number: usize) -> ExitCode {
    match get_dir_contents(&path) {
        Ok(files) => {
            for file in files {
                let original_file = file.path().file_stem().unwrap().to_os_string();
                let renamed_file = get_filenames(&file, number, &path);

                if let Err(e) = Command::new("mv")
                    .arg(&original_file)
                    .arg(&renamed_file)
                    .output()
                {
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
