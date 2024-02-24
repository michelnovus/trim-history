// [MIT License] Copyright (c) 2024 Michel Novus

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

#[cfg(test)]
mod tests {
    use super::*;

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
