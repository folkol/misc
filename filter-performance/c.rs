use std::io::{stdin, stdout, BufWriter, Result, Write};

fn main() -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let mut buffer = String::new();
    let mut acc = 0;
    loop {
        let n = stdin().read_line(&mut buffer)?;
        if n == 0 {
            break;
        }
        acc += buffer.len() - 1;
        write!(out, "{buffer}").unwrap();
        buffer.clear();
    }
    writeln!(out, "{acc}").unwrap();
    Ok(())
}
