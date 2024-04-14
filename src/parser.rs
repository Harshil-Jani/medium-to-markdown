use html_parser::{Dom, Node};

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
                    Some("".to_owned())
                }
                "i" => {
                    // Parse Italic
                    Some("".to_owned())
                }
                "br" => {
                    // Parse Line Break
                    Some("".to_owned())
                }
                "code" => {
                    // Parse `tag` Code
                    Some("".to_owned())
                }
                _ => {
                    // Check for nested elements
                    // Eg : <p> <b>Hello <i>World</i></b> </p>
                    // In order to check for nested elements, We need to recursively call extract_inline
                    // and iterate over the childrens of the inline elements.
                    Some("".to_owned())
                }
            }
        }
        _ => None,
    }
}
