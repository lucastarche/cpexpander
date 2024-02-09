# tarche's cpexpander

Inspired by [AtCoder's expander](https://github.com/atcoder/ac-library/blob/master/expander.py), this small Rust program can expand includes to submit to online judges.
The generated file is available as `000SUBMITME.cpp`, and will be created in the current working directory.

To compile the program, you need a working Rust installation. Then, run `cargo build --release`, and the output file will be in `target/release/cpexpander`
