use std::io;

pub fn ask(question: &str) -> Result<String, io::Error> {
    println!("{}", question);
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin
        .read_line(input)
        .expect("could not read input from stdin");
    Ok(input.trim_end().to_string())
}
