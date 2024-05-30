# Medium to Markdown Parser and CLI Tool

This Rust utility converts Medium blog posts into Markdown format, enabling users to preserve their content independently of centralized platforms. With this tool, you can safeguard your blog posts against potential risks associated with platform changes or shutdowns.

Medium provides a convenient platform for hosting blogs and reaching a wide audience. However, relying solely on Medium for content hosting carries risks. If Medium were to go down or change its policies, the hard work of blog writers would be at risk. To mitigate this risk, this project aims to provide a simple yet effective tool for converting Medium blog posts into Markdown format.

# Features
- [x] Headings (H1)
- [x] SubHeadings (H2)
- [x] Paragraphs (p)
- [x] Inline Tags this (code)
- [x] Code Blocks : Ctrl + Alt+6 The ones where you read code (pre)
- [x] Images (source)
- [x] Line Breaks (br)
- [x] Bold (b,strong), Italics (i)
- [x] Lists (li)

# Installation

Install directly from cargo.

```
cargo install medium-to-markdown
```

# Usage

The command-line interface (CLI) accepts a Medium blog post URL and a filename as input and generates the Markdown content.

```
medium-to-markdown <URL> <file_name>
```

Replace <URL> with the URL of the Medium blog post you want to convert, and <file_name> with the desired filename for the Markdown output.

## Example

```
medium-to-markdown https://medium.com/@harshiljani2002/building-stock-market-engine-from-scratch-in-rust-i-9be7c110e137 blog_post.md
```

# Developer Setup

To set up the project, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/Harshil-Jani/medium-to-markdown.git
```

2. Install Rust and Cargo if you haven't already.

3. Navigate to the project directory:

```
cd medium-to-markdown
```

4. Build the project:

```
cargo build --release
```

# Testing

The project includes unit tests to ensure the correctness of the parser. To run the tests, use the following command:

```
cargo test
```

# Contributions

Contributions to this project are welcome. If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request on GitHub.
