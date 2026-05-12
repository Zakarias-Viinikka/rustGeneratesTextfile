use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("./src/output.txt").unwrap();
    writeln!(file, "First 1asdds").unwrap();
    writeln!(file, "Second 2asdads: {}", 42).unwrap();
}
