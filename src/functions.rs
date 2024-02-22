// [MIT License] Copyright (c) 2024 Michel Novus

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
}