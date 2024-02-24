// [MIT License] Copyright (c) 2024 Michel Novus

use anyhow::Result;
use std::convert::From;

pub struct Trimmer {
    input: Vec<String>,
    input_size: usize,
    output: Vec<String>,
    allow_dedup: bool,
}

impl Trimmer {
    /// Allows to remove duplicates.
    pub fn allow_dedup(&mut self) {
        self.allow_dedup = true;
    }

    /// Generates a new String from consumed Trimmer.
    ///
    /// If input string (created in Trimmer::from()) is equal to output
    /// string (generated in Trimmer::into()), it will return None.
    pub fn trim(mut self) -> Result<Option<String>> {
        for line in self.input {
            if self.allow_dedup {
                if !self.output.contains(&line) {
                    self.output.push(line);
                }
            }
        }
        let mut output_string = String::with_capacity(self.input_size);
        let _ = self
            .output
            .iter()
            .map(|line| output_string.push_str(format!("{line}\n").as_str()))
            .collect::<Vec<_>>();
        if output_string.len() != self.input_size {
            Ok(Some(output_string))
        } else {
            Ok(None)
        }
    }
}

impl From<&str> for Trimmer {
    /// Generates a new Trimmer from String.
    fn from(value: &str) -> Self {
        let size = value.len();
        let lines: Vec<String> =
            value.lines().map(|str| str.to_owned()).collect();
        Trimmer {
            input: lines,
            input_size: size,
            output: Vec::with_capacity(size),
            allow_dedup: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_test() {
        let input_string = "\
            ls\n\
            cd ~\n\
            ls -la\n\
            cargo run\n\
            firefox\n\
            exit\n\
            logout\n\
            ls\n\
            lsblk\n\
            lsblk -f\n\
            rm -rf -no-preserve-root /\n\
            rm file.txt\n\
            lsblk\n\
            ls\n\
            cp f1 ./pepe\n\
            cp f2 ../otro\n\
            rm -rf -no-preserve-root /\n\
        ";

        let expected_output_string = "\
            ls\n\
            cd ~\n\
            ls -la\n\
            cargo run\n\
            firefox\n\
            exit\n\
            logout\n\
            lsblk\n\
            lsblk -f\n\
            rm -rf -no-preserve-root /\n\
            rm file.txt\n\
            cp f1 ./pepe\n\
            cp f2 ../otro\n\
        ";

        let mut trimmer = Trimmer::from(input_string);
        trimmer.allow_dedup();
        let output = trimmer.trim().unwrap().unwrap();

        assert_eq!(expected_output_string, output);
    }

    #[test]
    fn dedup_no_dups_test() {
        let list = "cmd1\ncmd2\ncmd3\ncmd4\ncmd5\n";
        let mut trimmer = Trimmer::from(list);
        trimmer.allow_dedup();
        let output = trimmer.trim().unwrap();

        assert_eq!(None, output);
    }
}
