use std::{
    fs,
    fs::{DirEntry, OpenOptions},
    path::Path,
};

use anyhow::{anyhow, bail, Result};
use console::style;

use crate::utils::{
    entry_to_file_name, get_padding_digits, get_sorted_files_in_dir, is_valid_name, num_prefix,
    rename_files_from_index,
};

pub fn process_insert(page_path: String) -> Result<()> {
    let page_path = Path::new(&page_path);
    let page_path_string = page_path.to_string_lossy().to_string();
    let page_name_os_str = page_path.file_name().ok_or_else(|| {
        anyhow!(
            "Couldn't get the file name of the given page path '{}'!",
            page_path_string
        )
    })?;
    let page_name_string = page_name_os_str.to_string_lossy().to_string();

    if !is_valid_name(&page_name_string) {
        bail!(
            "Invalid file name from the given filepath '{}!",
            page_path_string
        );
    };
    let mut parent_path = page_path.parent().unwrap_or_else(|| Path::new("."));
    if &parent_path.to_string_lossy() == "" {
        parent_path = Path::new(".");
    }
    let insert_prefix = num_prefix(&page_name_string)?;

    let all_files = get_sorted_files_in_dir(parent_path)?;
    let (digits_used, digits_needed) = get_padding_digits(&all_files);

    let (after_files, before_files): (Vec<DirEntry>, Vec<DirEntry>) =
        all_files.into_iter().partition(|f| {
            num_prefix(&entry_to_file_name(f)).unwrap_or_else(|_| {
                panic!("invalid file {} in sorted files", entry_to_file_name(f))
            }) >= insert_prefix
        });

    let mut renamed_count =
        rename_files_from_index(after_files, Some(insert_prefix), parent_path, digits_needed)?;
    if digits_used < digits_needed {
        renamed_count += rename_files_from_index(before_files, None, parent_path, digits_needed)?;
    }
    println!("Successfully renamed {} files!", renamed_count);

    let create_error = |err| {
        anyhow!(
            "Error creating the new Docusaurus file '{}' - {}",
            page_path.to_string_lossy(),
            err
        )
    };
    let fixed_path = parent_path.join(page_name_os_str);
    let mut insert_text = "page";
    if fixed_path.extension().is_none() {
        fs::create_dir(fixed_path).map_err(create_error)?;
        insert_text = "directory";
    } else {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(fixed_path)
            .map_err(create_error)?;
    }
    println!(
        "{}",
        style(format!("Successfully inserted a new {}!", insert_text))
            .green()
            .bold()
            .dim()
    );

    Ok(())
}
