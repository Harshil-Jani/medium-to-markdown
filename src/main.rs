use parser::parse_medium_post;

mod dom;
mod parser;

fn main() {
    let url = "https://www.rust-lang.org/";
    match dom::dom(url) {
        Ok(dom) => {}
        Err(e) => eprintln!("{}", e),
    };
}
