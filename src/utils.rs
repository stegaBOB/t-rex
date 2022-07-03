use std::{
    cmp::max,
    fs::{rename, DirEntry},
    io,
    path::Path,
};

use anyhow::{anyhow, bail, Result};

pub fn get_sorted_files_in_dir(directory_path: &Path) -> Result<Vec<DirEntry>> {
    if directory_path.is_file() {
        bail!(
            "Invalid directory path ('{}' is a file)!",
            directory_path.to_string_lossy()
        );
    }

    let mut files: Vec<DirEntry> = directory_path
        .read_dir()?
        .into_iter()
        .filter(|entry| {
            if let Ok(entry) = entry {
                is_valid_name(&entry.file_name().to_string_lossy())
            } else {
                false
            }
        })
        .collect::<Result<Vec<DirEntry>, io::Error>>()?;

    files.sort_by(|f1, f2| {
        num_prefix(&entry_to_file_name(f1))
            .expect("Not valid but already checked is_valid")
            .cmp(
                &num_prefix(&entry_to_file_name(f2))
                    .expect("Not valid but already checked is_valid"),
            )
    });

    Ok(files)
}

pub fn is_valid_name(file_name: &str) -> bool {
    is_numeric(&file_prefix(file_name))
}

pub fn file_prefix(file_name: &str) -> String {
    file_name
        .to_string()
        .split_once('-')
        .map_or(file_name.to_string(), |(start, _)| start.to_string())
}

pub fn is_numeric(file_prefix: &str) -> bool {
    file_prefix.chars().all(|c| c.is_digit(10))
}

pub fn num_prefix(file_name: &str) -> Result<usize> {
    let prefix = file_prefix(file_name);
    if is_numeric(&prefix) {
        Ok(prefix.parse()?)
    } else {
        Err(anyhow!("'{}' is an invalid file name!", file_name))
    }
}

pub fn entry_to_file_name(entry: &DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}

pub fn replaced_index_name_unchecked(name: &str, new_index: usize, padding: usize) -> String {
    let mut new_name = {
        let mut index = format!("{:0width$}", new_index, width = padding);
        index.push('-');
        index
    };
    let (_, old_end): (&str, &str) = name
        .split_once('-')
        .expect(&*format!("'{}' is an invalid file name!", name));
    new_name.push_str(old_end);

    new_name
}

pub fn rename_files_from_index(
    mut entries: Vec<DirEntry>,
    start_index: Option<usize>,
    parent_dir: &Path,
    padding: usize,
) -> Result<usize> {
    let mut new_names: Vec<String> = Vec::new();
    for (current_index, entry) in entries.iter().enumerate() {
        let file_name = entry_to_file_name(entry);
        let expected_index = if let Some(start_index) = &start_index {
            current_index + *start_index + 1
        } else {
            current_index
        };

        let new_name = if num_prefix(&file_name)? != expected_index {
            replaced_index_name_unchecked(&file_name, expected_index, padding)
        } else {
            file_name
        };

        new_names.push(new_name);
    }

    entries.reverse();
    new_names.reverse();

    let mut rename_count = 0;
    entries
        .into_iter()
        .enumerate()
        .try_for_each(|(index, entry)| {
            let entry_name = &entry_to_file_name(&entry);
            let new_name = &new_names[index];
            if entry_name != new_name {
                let new_path = parent_dir.join(new_name);
                rename_count += 1;
                rename(entry.path(), new_path).map_err(|err| {
                    anyhow!(
                        "Error renaming file '{}' to '{}' - {}",
                        entry_name,
                        new_name,
                        err
                    )
                })
            } else {
                Ok(())
            }
        })?;
    Ok(rename_count)
}

pub fn get_padding_digits(all_files: &[DirEntry]) -> (usize, usize) {
    let digits_needed = max((all_files.len() as f64).log10().ceil() as usize, 2);
    let digits_used = if let Some(file) = all_files.get(0) {
        file_prefix(&entry_to_file_name(file)).len()
    } else {
        0
    };
    (digits_used, digits_needed)
}
