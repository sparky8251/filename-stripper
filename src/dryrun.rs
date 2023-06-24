use std::{
    path::PathBuf,
    process::{Command, ExitCode},
};

use crate::utils::{get_dir_contents, get_filenames};

pub fn dryrun(path: PathBuf, number: usize) -> ExitCode {
    let mut dryrun_dir = path.clone();
    dryrun_dir.push("dryrun");

    if dryrun_dir.exists() && !dryrun_dir.is_dir() {
        eprintln!(
            "{:?} exists, but is not a directory. Dry run failed",
            dryrun_dir
        );
        return ExitCode::FAILURE;
    }

    if !dryrun_dir.exists() {
        if let Err(e) = Command::new("mkdir").arg(dryrun_dir.as_os_str()).output() {
            eprintln!("Unable to make dryrun dir due to error {:?}", e);
            return ExitCode::FAILURE;
        }
    }

    match get_dir_contents(&path) {
        Ok(files) => {
            for file in files {
                let original_file = file.path().file_stem().unwrap().to_os_string();
                let renamed_file = get_filenames(&file, number, &dryrun_dir);

                if let Err(e) = Command::new("ln")
                    .arg("-s")
                    .arg(&original_file)
                    .arg(&renamed_file)
                    .output()
                {
                    eprintln!(
                        "Unable to create symlink {:?} for original file {:?} due to error {}",
                        renamed_file, original_file, e
                    )
                }
            }
        }
        Err(e) => return e,
    };

    ExitCode::SUCCESS
}
