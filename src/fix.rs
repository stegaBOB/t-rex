use std::{fs::DirEntry, path::Path};

use anyhow::{bail, Result};
use console::style;

use crate::utils::{get_padding_digits, get_sorted_files_in_dir, rename_files_from_index};

pub fn process_fix(dir_path: String) -> Result<()> {
    let dir_path = Path::new(&dir_path);
    let dir_path_string = dir_path.to_string_lossy().to_string();
    if !dir_path.is_dir() {
        bail!(
            "Couldn't find the directory with the given path '{}'!",
            dir_path_string
        );
    }
    let sorted_files: Vec<DirEntry> = get_sorted_files_in_dir(dir_path)?;
    let (_, digits_needed) = get_padding_digits(&sorted_files);
    let fix_count = rename_files_from_index(sorted_files, None, dir_path, digits_needed)?;

    println!(
        "{}",
        style(format!("Successfully fixed {} files!", fix_count))
            .green()
            .bold()
            .dim()
    );

    Ok(())
}
