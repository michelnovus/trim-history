// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::Result;
use std::fs::{read, write};
use std::path::PathBuf;

/// Check if path of file contains a dot in name start and history word.
pub fn is_histfile(path: &PathBuf) -> bool {
    let filename = path.file_name();
    if filename.is_none() {
        return false;
    }
    let filename = filename.unwrap().to_str().unwrap();
    filename.contains("history") && filename.starts_with(".")
}

/// Read text file and split it in lines.
pub fn readlines(path: &PathBuf) -> Result<Vec<String>> {
    let raw_file_content = read(path)?;
    let formated_file_content = String::from_utf8_lossy(&raw_file_content);
    let lines: Vec<String> = formated_file_content
        .lines()
        .map(|str| str.to_owned())
        .collect();
    Ok(lines)
}

/// Write each element of vector as a new line of text file.
pub fn writelines(path: &PathBuf, lines: &Vec<&str>) -> Result<()> {
    let mut unified_lines = String::new();
    let _ = lines
        .iter()
        .map(|line| unified_lines.push_str(format!("{line}\n").as_str()))
        .collect::<Vec<_>>();
    write(path, unified_lines)?;
    Ok(())
}

/// Removes all duplicate elements in string list.
///
/// Function accept a vector of string slice and return a new vector
/// of string slice where every string is unique and mantains order of
/// first ocurrence. If both vectors are equals the function returns None.
pub fn rm_dups<'a>(list: &Vec<&'a str>) -> Option<Vec<&'a str>> {
    let mut output_list: Vec<&str> = Vec::new();
    for in_elem in list {
        if !output_list.contains(&in_elem) {
            output_list.push(*in_elem);
        }
    }
    if list.len() == output_list.len() {
        None
    } else {
        Some(output_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm_dups_test() {
        let list_dups = vec![
            "ls",
            "cd ~",
            "ls -la",
            "cargo run",
            "firefox",
            "exit",
            "logout",
            "ls",
            "lsblk",
            "lsblk -f",
            "rm -rf -no-preserve-root /",
            "rm file.txt",
            "lsblk",
            "ls",
            "cp f1 ./pepe",
            "cp f2 ../otro",
            "rm -rf -no-preserve-root /",
        ];
        let list_expected = vec![
            "ls",
            "cd ~",
            "ls -la",
            "cargo run",
            "firefox",
            "exit",
            "logout",
            "lsblk",
            "lsblk -f",
            "rm -rf -no-preserve-root /",
            "rm file.txt",
            "cp f1 ./pepe",
            "cp f2 ../otro",
        ];

        assert_eq!(list_expected, rm_dups(&list_dups).unwrap())
    }

    #[test]
    fn rm_dups_without_dups_test() {
        let list = vec!["cmd1", "cmd2", "cmd3", "cmd4", "cmd5"];
        assert_eq!(None, rm_dups(&list));
    }

    #[test]
    fn is_histfile_test() {
        let paths = vec![
            PathBuf::from(".history"),
            PathBuf::from(".something"),
            PathBuf::from("history_event"),
            PathBuf::from(".bash_history"),
            PathBuf::from(".passwords"),
            PathBuf::from("secrets"),
            PathBuf::from("file.history.txt"),
            PathBuf::from(".passwords_history"),
        ];
        let expected =
            vec![true, false, false, true, false, false, false, true];
        paths
            .iter()
            .zip(expected.iter())
            .map(|(path, exp)| assert_eq!(*exp, is_histfile(path)))
            .collect()
    }
}
