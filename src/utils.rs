use std::fs::DirEntry;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn get_dir_contents(path: &PathBuf) -> Result<impl Iterator<Item = DirEntry>, ExitCode> {
    let files = match path.read_dir() {
        Ok(v) => v.filter_map(|x| x.ok()).filter_map(|x| {
            if let Ok(entry) = x.file_type() {
                if entry.is_file() {
                    Some(x)
                } else {
                    None
                }
            } else {
                None
            }
        }),
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

pub fn get_filenames(
    file: &DirEntry,
    number: usize,
    origin_dir: &PathBuf,
    dest_dir: &PathBuf,
) -> (PathBuf, PathBuf) {
    let filename = String::from(file.path().file_name().unwrap().to_string_lossy());
    let trimmed_filename: String = filename.chars().skip(number).collect();

    let mut original_file = origin_dir.clone();
    original_file.push(filename);

    let mut renamed_file = dest_dir.clone();
    renamed_file.push(trimmed_filename);

    (original_file, renamed_file)
}
