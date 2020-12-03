use std::fs::*;

fn main() {
    println!("cargo:rerun-if-changed=input.txt");
    let input = read_to_string("input.txt").unwrap();
    // under-estimated but better than nothing
    let mut rs = String::with_capacity(input.len() * 2);
    for line in input.lines() {
        rs.push('[');
        for c in line.chars() {
            rs.push_str(match c { '.' => ". ", '#' => "# ", _ => unreachable!() });
        }
        rs.push_str("]\n");
    }

    let mut out = File::create(format!("{}/input_txt.rs", std::env::var("OUT_DIR").unwrap())).unwrap();
    use std::io::Write;
    writeln!(&mut out, "type Input = input!(\n{});", rs).unwrap();
}
