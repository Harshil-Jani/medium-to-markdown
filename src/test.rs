#[cfg(test)]
mod test {
    use crate::parser::*;
    use html_parser::Dom;

    #[test]
    fn test_heading() {
        let html = "<html><head></head><body><h1>Hii</h1></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(markdown, "# Hii\n".to_string());
        }

        let html = "<html><head></head><body><h2>Hii</h2></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(markdown, "## Hii\n".to_string());
        }
    }

    #[test]
    fn test_paragraph() {
        let html = "<html><head></head><body><p>Hii</p></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(markdown, "".to_string());
        }

        let html =
            "<html><head></head><body><p class='pw-post-body-paragraph'>Hii</p></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(markdown, "\nHii\n".to_string());
        }
    }

    #[test]
    fn test_code_snippets() {
        let html = "<html><head></head><body><pre>fn main() { println!(\"Hello, World!\"); }</pre></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(
                markdown,
                "```\nfn main() { println!(\"Hello, World!\"); }\n```\n".to_string()
            );
        }
    }

    #[test]
    fn test_image() {
        let html = "<html><head></head><body><source srcSet='https://example.com/image.webp'></source></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(
                markdown,
                "![Medium-Image](https://example.com/image.webp)\n".to_string()
            );
        }
    }

    #[test]
    fn test_nested_elements() {
        let html = "<html><head></head><body><h1>Heading</h1><p class='pw-post-body-paragraph'>Paragraph</p></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(markdown, "# Heading\n\nParagraph\n".to_string());
        }
    }

    #[test]
    fn test_text_formatting() {
        let html = "<html><head></head><body><h1>Heading</h1><p class='pw-post-body-paragraph'>Paragraph <strong>bold</strong> <em>italic</em></p></body></html>";
        if let Ok(dom) = Dom::parse(&html) {
            let markdown = parse_medium_post(dom).unwrap();
            assert_eq!(
                markdown,
                "# Heading\n\nParagraph **bold** *italic*\n".to_string()
            );
        }
    }
}
