use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn get_dir_contents(path: &PathBuf) -> Result<impl Iterator<Item = DirEntry>, ExitCode> {
    let files = match path.read_dir() {
        Ok(v) => v
            .flatten()
            .filter(|x| x.file_type().map(|f| f.is_file()).unwrap_or(false)),
        Err(e) => {
            eprintln!(
                "Unable to read files in path {:?} due to error {:?}",
                path, e
            );
            return Err(ExitCode::FAILURE);
        }
    };
    Ok(files)
}

pub fn get_filenames(file: &DirEntry, number: usize, dest_dir: &Path) -> PathBuf {
    let filename = String::from(file.path().file_name().unwrap().to_string_lossy());
    let trimmed_filename: String = filename.chars().skip(number).collect();
    dest_dir.join(trimmed_filename)
}
