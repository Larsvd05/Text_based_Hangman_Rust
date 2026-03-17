# README for text based hangman written in Rust

## Background

This project was written to both improve my knowledge of coding in Rust, but also act as an example of what my code looks like to show to companies.

## User manual for running the program

If you don't have the executable, then you need to build it yourself.
A requirement for this is that you have Rust already installed on your system. This can be checked by using `cargo --version`, which should return a valid version (in my case cargo 1.94.0).

Now on to actually building and running the program.
First off, make sure you are in the root directory, this is Text_based_Hangman_Rust.
This can be checked by making sure you have direct access to the src folder by using `ls src/`. If you receive an error message saying "No such file or directory", you are in the wrong directory and thus not in the root of this project.

There are multiple ways to build or run the program, but the easiest is using `cargo r`, which if short for `cargo run`. This will automatically build and run your code.

You can also only build it by using `cargo b`, which is short for `cargo build`.
