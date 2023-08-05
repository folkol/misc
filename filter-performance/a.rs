use std::io::stdin;

fn main() {
    let mut acc = 0usize;
    for line in stdin().lines().map_while(Result::ok) {
        acc += line.len();
        println!("{line}");
    }
    println!("{acc}");
}
