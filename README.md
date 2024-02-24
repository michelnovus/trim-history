# Trim History

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

### What is it?

It's a program that trims all duplicate lines in the shell history file.

I'll probably add delete chosen commands functionality and other things
whenever i can.

### Build program

Getting an executable from the Rust source code is very easy!  

```
git clone 'https://github.com/michelnovus/trim-history'  
cd trim-history  
cargo build --release
```

Now, inside `target/release` you will find the executable called `trim-history`.

### Install the execution hook

Add somewhere in the shell *rc file* the line:

```
trap "/path/to/trim-history-executable" EXIT
```
