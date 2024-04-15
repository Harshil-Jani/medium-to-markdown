use html_parser::{Dom, Element, Node};

pub fn parse_medium_post(dom: Dom) {
    for node in &dom.children {
        // Iterate thorough nodes in Dom
        parse_dom(node);
    }
}

fn parse_dom(node: &Node) {
    match node.element() {
        Some(element) => {
            match element.name.as_str() {
                "h1" => {
                    // Parse Heading
                }
                "h2" => {
                    // Parse Subheading
                }
                "p" => {
                    // Parse Paragraph
                }
                "pre" => {
                    // Parse Code Block
                }
                "source" => {
                    // Parse Image
                }
                _ => {}
            }
        }
        None => {}
    }
}

fn extract_inline(node: &Node) -> Option<String> {
    match node {
        Node::Text(text) => Some(text.to_owned()),
        Node::Element(element) => {
            match element.name.as_str() {
                "b" | "strong" => {
                    // Parse Bold
                    let text = parse_formatting_text(element);
                    Some(format!("**{}** ", text.trim()))
                }
                "i" => {
                    // Parse Italic
                    let text = parse_formatting_text(element);
                    Some(format!("*{}* ", text.trim()))
                }
                "br" => {
                    // Parse Line Break
                    Some("\n".to_owned())
                }
                "code" => {
                    // Parse `tag` Code
                    let text = parse_formatting_text(element);
                    Some(format!("`{}` ", text.trim()))
                }
                _ => {
                    // Check for nested elements
                    // Eg : <p> <b>Hello <i>World</i></b> </p>
                    // In order to check for nested elements, We need to recursively call extract_inline
                    // and iterate over the childrens of the inline elements.
                    // If the text is not formatted then return as a simple text.
                    let text = parse_formatting_text(element);
                    Some(text)
                }
            }
        }
        _ => None,
    }
}

fn parse_formatting_text(element : &Element) -> String{
    let mut result = String::new();
    for child in &element.children {
        if let Some(text) = extract_inline(child) {
            result.push_str(&text);
        }
    }
    result
}