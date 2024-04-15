use html_parser::{Dom, Element, Node};

pub fn parse_medium_post(dom: Dom) -> Result<String, Box<dyn std::error::Error>> {
    let mut parsed_post = String::new();
    for node in &dom.children {
        // Iterate thorough nodes in Dom
        if let Some(content) = parse_dom(node) {
            parsed_post.push_str(&content);
        }
    }
    Ok(parsed_post)
}

fn parse_dom(node: &Node) -> Option<String> {
    let mut parsed_content = String::new();

    match node.element() {
        Some(element) => match element.name.as_str() {
            "h1" => {
                if let Some(text) = extract_inline(node) {
                    parsed_content.push_str(&format!("# {}\n", text));
                }
            }
            "h2" => {
                if let Some(text) = extract_inline(node) {
                    parsed_content.push_str(&format!("## {}\n", text));
                }
            }
            "p" => {
                if !element.classes.is_empty() && element.classes[0] == "pw-post-body-paragraph" {
                    if let Some(text) = extract_inline(node) {
                        parsed_content.push_str(&format!("\n{}\n", text.trim()));
                    }
                }
            }
            "pre" => {
                if let Some(text) = extract_inline(node) {
                    parsed_content.push_str(&format!("```\n{}\n```\n", text));
                }
            }
            "source" => {
                if let Some(links_str) = element.attributes.get("srcSet") {
                    match links_str {
                        Some(links_str) => {
                            let links = links_str.split(" ").collect::<Vec<&str>>();
                            if let Some(link) = links.first().filter(|link| link.contains("webp")) {
                                parsed_content
                                    .push_str(&format!("![Medium-Image]({})\n", link.trim()));
                            }
                        }
                        None => {}
                    }
                }
            }

            _ => {
                for child in &element.children {
                    if let Some(child_content) = parse_dom(child) {
                        parsed_content.push_str(&child_content);
                    }
                }
            }
        },
        None => {}
    };

    if parsed_content.is_empty() {
        None
    } else {
        Some(parsed_content)
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
                "i" | "em" => {
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

fn parse_formatting_text(element: &Element) -> String {
    let mut result = String::new();
    for child in &element.children {
        if let Some(text) = extract_inline(child) {
            result.push_str(&text);
        }
    }
    result
}
