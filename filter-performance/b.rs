use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout());
    let mut acc = 0;
    for line in stdin().lines().map_while(Result::ok) {
        acc += line.len();
        writeln!(out, "{line}").unwrap();
    }
    writeln!(out, "{acc}").unwrap();
}
