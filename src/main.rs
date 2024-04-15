use std::env;
use std::fs;
use std::io::Write;

mod dom;
mod parser;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <URL> <file_name>", args[0]);
        return;
    }

    let url = &args[1];
    let file_name = &args[2];

    match dom::dom(url) {
        Ok(dom) => {
            if let Ok(markdown) = parser::parse_medium_post(dom) {
                if let Err(err) = write_to_file(file_name, &markdown) {
                    eprintln!("Error writing to file: {}", err);
                }
            } else {
                eprintln!("Error parsing Medium post");
            }
        }
        Err(e) => eprintln!("{}", e),
    };
}

fn write_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
