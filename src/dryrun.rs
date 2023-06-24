use std::{
    fs::create_dir,
    io::{self, ErrorKind},
    os::unix::fs::symlink,
    path::Path,
};

use crate::utils::{get_filenames, get_files};

pub fn dryrun(path: &Path, number: usize) -> Result<(), io::Error> {
    let dryrun_dir = path.join("dryrun");

    if let Err(e) = create_dir(&dryrun_dir) {
        match e.kind() {
            ErrorKind::AlreadyExists => {
                if !dryrun_dir.is_dir() {
                    eprintln!(
                        "dryrun dir {:?} exists already and is not a directory, cannot continue",
                        dryrun_dir
                    );
                    return Err(io::Error::from(ErrorKind::AlreadyExists));
                }
            }
            _ => return Err(e),
        }
    }

    match get_files(path) {
        Ok(files) => {
            for file in files {
                let original_file = file.path();
                let renamed_file = get_filenames(&file, number, &dryrun_dir);

                if let Err(e) = symlink(&original_file, &renamed_file) {
                    eprintln!(
                        "Unable to create symlink {:?} for original file {:?} due to error {}",
                        renamed_file, original_file, e
                    )
                }
            }
        }
        Err(e) => return Err(e),
    };

    Ok(())
}
