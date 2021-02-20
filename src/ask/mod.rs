use std::io;

pub fn ask(question: &str) -> Result<String, io::Error> {
    println!("{}", question);
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input)?;
    Ok(input.trim_end().to_string())
}
