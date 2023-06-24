use std::{
    fs::DirEntry,
    io,
    path::{Path, PathBuf},
};

pub fn get_files(path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    let files = path
        .read_dir()?
        .flatten()
        .filter(|x| x.file_type().map(|f| f.is_file()).unwrap_or(false))
        .collect();
    Ok(files)
}

pub fn get_filenames(file: &DirEntry, number: usize, dest_dir: &Path) -> PathBuf {
    let filename = String::from(file.path().file_name().unwrap().to_string_lossy());
    let trimmed_filename: String = filename.chars().skip(number).collect();
    dest_dir.join(trimmed_filename)
}
