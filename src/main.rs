mod dom;

fn main() {
    let url = "https://www.rust-lang.org/";
    match dom::dom(url) {
        Ok(dom) => {todo!()}
        Err(e) => eprintln!("{}", e),
    }
}
