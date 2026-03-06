use markdown::*;

// ===== Topic 1: Inline Parsing =====

#[test]
fn test_plain_text() {
    let result = parse_inline("hello world");
    assert_eq!(result, vec![Inline::Text("hello world".to_string())]);
}

#[test]
fn test_bold() {
    let result = parse_inline("**bold**");
    assert_eq!(
        result,
        vec![Inline::Bold(vec![Inline::Text("bold".to_string())])]
    );
}

#[test]
fn test_italic() {
    let result = parse_inline("*italic*");
    assert_eq!(
        result,
        vec![Inline::Italic(vec![Inline::Text("italic".to_string())])]
    );
}

#[test]
fn test_inline_code() {
    let result = parse_inline("`code`");
    assert_eq!(result, vec![Inline::Code("code".to_string())]);
}

#[test]
fn test_link() {
    let result = parse_inline("[click](https://example.com)");
    assert_eq!(
        result,
        vec![Inline::Link {
            text: "click".to_string(),
            url: "https://example.com".to_string(),
        }]
    );
}

#[test]
fn test_image() {
    let result = parse_inline("![alt](img.png)");
    assert_eq!(
        result,
        vec![Inline::Image {
            alt: "alt".to_string(),
            url: "img.png".to_string(),
        }]
    );
}

#[test]
fn test_mixed_inline() {
    let result = parse_inline("hello **world** and *italic*");
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], Inline::Text("hello ".to_string()));
}

// ===== Topic 2 & 3: Block Parsing =====

#[test]
fn test_heading() {
    let blocks = parse_blocks("# Title");
    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].is_heading());
    assert_eq!(blocks[0].heading_level(), Some(1));
}

#[test]
fn test_heading_levels() {
    let blocks = parse_blocks("## Sub\n### Sub-sub");
    assert_eq!(blocks[0].heading_level(), Some(2));
    assert_eq!(blocks[1].heading_level(), Some(3));
}

#[test]
fn test_paragraph() {
    let blocks = parse_blocks("Hello world.\nSecond line.");
    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].is_paragraph());
}

#[test]
fn test_code_block() {
    let blocks = parse_blocks("```rust\nlet x = 1;\n```");
    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].is_code_block());
    match &blocks[0] {
        Block::CodeBlock(lang, code) => {
            assert_eq!(lang, "rust");
            assert_eq!(code, "let x = 1;");
        }
        _ => panic!("expected code block"),
    }
}

#[test]
fn test_unordered_list() {
    let blocks = parse_blocks("- item 1\n- item 2\n- item 3");
    match &blocks[0] {
        Block::UnorderedList(items) => assert_eq!(items.len(), 3),
        _ => panic!("expected unordered list"),
    }
}

#[test]
fn test_ordered_list() {
    let blocks = parse_blocks("1. first\n2. second\n3. third");
    match &blocks[0] {
        Block::OrderedList(items) => assert_eq!(items.len(), 3),
        _ => panic!("expected ordered list"),
    }
}

#[test]
fn test_blockquote() {
    let blocks = parse_blocks("> Hello\n> World");
    match &blocks[0] {
        Block::BlockQuote(inner) => {
            assert_eq!(inner.len(), 1); // merged into paragraph
        }
        _ => panic!("expected blockquote"),
    }
}

#[test]
fn test_horizontal_rule() {
    let blocks = parse_blocks("---");
    assert_eq!(blocks, vec![Block::HorizontalRule]);
}

// ===== Topic 4: HTML Rendering =====

#[test]
fn test_render_heading() {
    assert_eq!(render("# Hello"), "<h1>Hello</h1>");
}

#[test]
fn test_render_paragraph() {
    assert_eq!(render("Hello world"), "<p>Hello world</p>");
}

#[test]
fn test_render_bold_italic() {
    assert_eq!(
        render("**bold** and *italic*"),
        "<p><strong>bold</strong> and <em>italic</em></p>"
    );
}

#[test]
fn test_render_code_block() {
    let html = render("```js\nalert(1)\n```");
    assert!(html.contains("language-js"));
    assert!(html.contains("alert(1)"));
}

#[test]
fn test_render_link() {
    let html = render("[Rust](https://rust-lang.org)");
    assert!(html.contains("<a href=\"https://rust-lang.org\">Rust</a>"));
}

#[test]
fn test_render_list() {
    let html = render("- alpha\n- beta");
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>alpha</li>"));
    assert!(html.contains("<li>beta</li>"));
}

#[test]
fn test_escape_html() {
    assert_eq!(escape_html("<script>"), "&lt;script&gt;");
    assert_eq!(escape_html("a & b"), "a &amp; b");
}

#[test]
fn test_render_hr() {
    assert_eq!(render("---"), "<hr />");
}

// ===== Topic 5: Document Extraction =====

#[test]
fn test_extract_headings() {
    let blocks = parse_blocks("# Title\nText\n## Sub");
    let headings = extract_headings(&blocks);
    assert_eq!(headings.len(), 2);
    assert_eq!(headings[0], (1, "Title".to_string()));
    assert_eq!(headings[1], (2, "Sub".to_string()));
}

#[test]
fn test_extract_links() {
    let blocks = parse_blocks("Visit [Rust](https://rust-lang.org) or [Go](https://go.dev)");
    let links = extract_links(&blocks);
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].1, "https://rust-lang.org");
}

#[test]
fn test_word_count() {
    let blocks = parse_blocks("# Hello World\n\nThis is a paragraph with seven words.");
    let wc = word_count(&blocks);
    assert_eq!(wc, 9); // 2 from heading + 7 from paragraph
}

// ===== Topic 6: TOC =====

#[test]
fn test_slugify() {
    assert_eq!(slugify("Hello World"), "hello-world");
    assert_eq!(slugify("What's New?"), "what-s-new");
}

#[test]
fn test_generate_toc() {
    let blocks = parse_blocks("# Intro\n## Setup\n## Usage");
    let toc = generate_toc(&blocks);
    assert_eq!(toc.len(), 3);
    assert_eq!(toc[0].slug, "intro");
    assert_eq!(toc[1].level, 2);
}

#[test]
fn test_render_toc() {
    let blocks = parse_blocks("# Title\n## Section");
    let toc = generate_toc(&blocks);
    let rendered = render_toc(&toc);
    assert!(rendered.contains("- [Title](#title)"));
    assert!(rendered.contains("  - [Section](#section)"));
}
