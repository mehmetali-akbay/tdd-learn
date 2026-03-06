// ============================================
// Level 6, Project 4: Markdown Parser
// Learn: Parsing, AST, recursion, HTML rendering
// ============================================

// ============================================
// Topic 1: Inline Elements — Bold, Italic, Code, Links
// Learn: Parsing inline markup, nested elements
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Code(String),
    Link { text: String, url: String },
    Image { alt: String, url: String },
}

/// Parse inline markdown elements from a string
pub fn parse_inline(input: &str) -> Vec<Inline> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buf = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '`' => {
                if !text_buf.is_empty() {
                    result.push(Inline::Text(text_buf.clone()));
                    text_buf.clear();
                }
                chars.next();
                let mut code = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '`' {
                        chars.next();
                        break;
                    }
                    code.push(c);
                    chars.next();
                }
                result.push(Inline::Code(code));
            }
            '*' => {
                chars.next();
                if chars.peek() == Some(&'*') {
                    // Bold **...**
                    chars.next();
                    if !text_buf.is_empty() {
                        result.push(Inline::Text(text_buf.clone()));
                        text_buf.clear();
                    }
                    let mut inner = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '*' {
                            chars.next();
                            if chars.peek() == Some(&'*') {
                                chars.next();
                                break;
                            }
                            inner.push('*');
                        } else {
                            inner.push(c);
                            chars.next();
                        }
                    }
                    result.push(Inline::Bold(vec![Inline::Text(inner)]));
                } else {
                    // Italic *...*
                    if !text_buf.is_empty() {
                        result.push(Inline::Text(text_buf.clone()));
                        text_buf.clear();
                    }
                    let mut inner = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '*' {
                            chars.next();
                            break;
                        }
                        inner.push(c);
                        chars.next();
                    }
                    result.push(Inline::Italic(vec![Inline::Text(inner)]));
                }
            }
            '[' => {
                chars.next();
                if !text_buf.is_empty() {
                    result.push(Inline::Text(text_buf.clone()));
                    text_buf.clear();
                }
                let mut link_text = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        chars.next();
                        break;
                    }
                    link_text.push(c);
                    chars.next();
                }
                if chars.peek() == Some(&'(') {
                    chars.next();
                    let mut url = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == ')' {
                            chars.next();
                            break;
                        }
                        url.push(c);
                        chars.next();
                    }
                    result.push(Inline::Link {
                        text: link_text,
                        url,
                    });
                } else {
                    text_buf.push('[');
                    text_buf.push_str(&link_text);
                    text_buf.push(']');
                }
            }
            '!' if {
                let mut peek = chars.clone();
                peek.next();
                peek.peek() == Some(&'[')
            } =>
            {
                chars.next(); // skip !
                chars.next(); // skip [
                if !text_buf.is_empty() {
                    result.push(Inline::Text(text_buf.clone()));
                    text_buf.clear();
                }
                let mut alt = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        chars.next();
                        break;
                    }
                    alt.push(c);
                    chars.next();
                }
                if chars.peek() == Some(&'(') {
                    chars.next();
                    let mut url = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == ')' {
                            chars.next();
                            break;
                        }
                        url.push(c);
                        chars.next();
                    }
                    result.push(Inline::Image { alt, url });
                }
            }
            _ => {
                text_buf.push(ch);
                chars.next();
            }
        }
    }

    if !text_buf.is_empty() {
        result.push(Inline::Text(text_buf));
    }

    result
}

// ============================================
// Topic 2: Block Elements — AST
// Learn: Headings, paragraphs, lists, code blocks, blockquotes
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Heading(u8, Vec<Inline>),        // level 1-6, content
    Paragraph(Vec<Inline>),          // inline content
    CodeBlock(String, String),       // language, code
    BlockQuote(Vec<Block>),          // nested blocks
    UnorderedList(Vec<Vec<Inline>>), // list items
    OrderedList(Vec<Vec<Inline>>),   // numbered items
    HorizontalRule,
    BlankLine,
}

impl Block {
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            Block::Heading(level, _) => Some(*level),
            _ => None,
        }
    }

    pub fn is_heading(&self) -> bool {
        matches!(self, Block::Heading(_, _))
    }

    pub fn is_paragraph(&self) -> bool {
        matches!(self, Block::Paragraph(_))
    }

    pub fn is_code_block(&self) -> bool {
        matches!(self, Block::CodeBlock(_, _))
    }
}

// ============================================
// Topic 3: Block Parser
// Learn: Line-based parsing, state machine
// ============================================

/// Parse markdown text into a list of blocks
pub fn parse_blocks(input: &str) -> Vec<Block> {
    let lines: Vec<&str> = input.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Blank line
        if line.trim().is_empty() {
            i += 1;
            continue;
        }

        // Horizontal rule
        if line.trim().len() >= 3
            && line
                .trim()
                .chars()
                .all(|c| c == '-' || c == '*' || c == '_')
            && line
                .trim()
                .chars()
                .next()
                .map(|c| line.trim().chars().filter(|&x| x == c).count() >= 3)
                .unwrap_or(false)
        {
            blocks.push(Block::HorizontalRule);
            i += 1;
            continue;
        }

        // Heading
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count() as u8;
            if level <= 6 {
                let text = line[level as usize..].trim();
                blocks.push(Block::Heading(level, parse_inline(text)));
                i += 1;
                continue;
            }
        }

        // Code block
        if line.trim().starts_with("```") {
            let lang = line.trim().trim_start_matches('`').trim().to_string();
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                code_lines.push(lines[i]);
                i += 1;
            }
            i += 1; // skip closing ```
            blocks.push(Block::CodeBlock(lang, code_lines.join("\n")));
            continue;
        }

        // Blockquote
        if line.starts_with("> ") || line == ">" {
            let mut quote_lines = Vec::new();
            while i < lines.len() && (lines[i].starts_with("> ") || lines[i] == ">") {
                let content = if lines[i] == ">" { "" } else { &lines[i][2..] };
                quote_lines.push(content);
                i += 1;
            }
            let inner = quote_lines.join("\n");
            blocks.push(Block::BlockQuote(parse_blocks(&inner)));
            continue;
        }

        // Unordered list
        if line.starts_with("- ") || line.starts_with("* ") {
            let mut items = Vec::new();
            while i < lines.len() && (lines[i].starts_with("- ") || lines[i].starts_with("* ")) {
                items.push(parse_inline(&lines[i][2..]));
                i += 1;
            }
            blocks.push(Block::UnorderedList(items));
            continue;
        }

        // Ordered list
        if line
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && line.contains(". ")
        {
            let mut items = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if let Some(dot_pos) = l.find(". ") {
                    if l[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
                        items.push(parse_inline(&l[dot_pos + 2..]));
                        i += 1;
                        continue;
                    }
                }
                break;
            }
            blocks.push(Block::OrderedList(items));
            continue;
        }

        // Paragraph: collect consecutive non-empty lines
        let mut para_lines = Vec::new();
        while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].starts_with('#') {
            para_lines.push(lines[i]);
            i += 1;
        }
        let text = para_lines.join(" ");
        blocks.push(Block::Paragraph(parse_inline(&text)));
    }

    blocks
}

// ============================================
// Topic 4: HTML Renderer
// Learn: AST to HTML conversion, escaping
// ============================================

/// Escape HTML special characters
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Render inline elements to HTML
pub fn render_inline(elements: &[Inline]) -> String {
    elements
        .iter()
        .map(|el| match el {
            Inline::Text(t) => escape_html(t),
            Inline::Bold(inner) => format!("<strong>{}</strong>", render_inline(inner)),
            Inline::Italic(inner) => format!("<em>{}</em>", render_inline(inner)),
            Inline::Code(code) => format!("<code>{}</code>", escape_html(code)),
            Inline::Link { text, url } => {
                format!("<a href=\"{}\">{}</a>", escape_html(url), escape_html(text))
            }
            Inline::Image { alt, url } => {
                format!(
                    "<img src=\"{}\" alt=\"{}\" />",
                    escape_html(url),
                    escape_html(alt)
                )
            }
        })
        .collect()
}

/// Render a block to HTML
pub fn render_block(block: &Block) -> String {
    match block {
        Block::Heading(level, content) => {
            format!("<h{}>{}</h{}>", level, render_inline(content), level)
        }
        Block::Paragraph(content) => {
            format!("<p>{}</p>", render_inline(content))
        }
        Block::CodeBlock(lang, code) => {
            if lang.is_empty() {
                format!("<pre><code>{}</code></pre>", escape_html(code))
            } else {
                format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>",
                    escape_html(lang),
                    escape_html(code)
                )
            }
        }
        Block::BlockQuote(inner) => {
            let content: String = inner.iter().map(render_block).collect();
            format!("<blockquote>{}</blockquote>", content)
        }
        Block::UnorderedList(items) => {
            let li: String = items
                .iter()
                .map(|item| format!("<li>{}</li>", render_inline(item)))
                .collect();
            format!("<ul>{}</ul>", li)
        }
        Block::OrderedList(items) => {
            let li: String = items
                .iter()
                .map(|item| format!("<li>{}</li>", render_inline(item)))
                .collect();
            format!("<ol>{}</ol>", li)
        }
        Block::HorizontalRule => "<hr />".to_string(),
        Block::BlankLine => String::new(),
    }
}

/// Render a full markdown document to HTML
pub fn render(input: &str) -> String {
    let blocks = parse_blocks(input);
    blocks
        .iter()
        .map(render_block)
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================
// Topic 5: Document Structure — Extraction
// Learn: Extracting metadata from parsed documents
// ============================================

/// Extract all headings from a document
pub fn extract_headings(blocks: &[Block]) -> Vec<(u8, String)> {
    blocks
        .iter()
        .filter_map(|block| match block {
            Block::Heading(level, content) => Some((*level, render_inline(content))),
            _ => None,
        })
        .collect()
}

/// Extract all links from a document
pub fn extract_links(blocks: &[Block]) -> Vec<(String, String)> {
    let mut links = Vec::new();
    for block in blocks {
        match block {
            Block::Paragraph(inlines) | Block::Heading(_, inlines) => {
                extract_links_from_inline(inlines, &mut links);
            }
            Block::UnorderedList(items) | Block::OrderedList(items) => {
                for item in items {
                    extract_links_from_inline(item, &mut links);
                }
            }
            Block::BlockQuote(inner) => {
                links.extend(extract_links(inner));
            }
            _ => {}
        }
    }
    links
}

fn extract_links_from_inline(inlines: &[Inline], links: &mut Vec<(String, String)>) {
    for inline in inlines {
        match inline {
            Inline::Link { text, url } => links.push((text.clone(), url.clone())),
            Inline::Bold(inner) | Inline::Italic(inner) => {
                extract_links_from_inline(inner, links);
            }
            _ => {}
        }
    }
}

/// Count words in a document (text only)
pub fn word_count(blocks: &[Block]) -> usize {
    let mut count = 0;
    for block in blocks {
        match block {
            Block::Paragraph(inlines) | Block::Heading(_, inlines) => {
                count += count_inline_words(inlines);
            }
            Block::UnorderedList(items) | Block::OrderedList(items) => {
                for item in items {
                    count += count_inline_words(item);
                }
            }
            Block::BlockQuote(inner) => {
                count += word_count(inner);
            }
            _ => {}
        }
    }
    count
}

fn count_inline_words(inlines: &[Inline]) -> usize {
    let mut count = 0;
    for inline in inlines {
        match inline {
            Inline::Text(t) => count += t.split_whitespace().count(),
            Inline::Bold(inner) | Inline::Italic(inner) => count += count_inline_words(inner),
            Inline::Code(c) => count += c.split_whitespace().count(),
            Inline::Link { text, .. } => count += text.split_whitespace().count(),
            Inline::Image { .. } => {}
        }
    }
    count
}

// ============================================
// Topic 6: Table of Contents Generator
// Learn: Building structure from flat data
// ============================================

/// A table of contents entry
#[derive(Debug, Clone, PartialEq)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub slug: String,
}

/// Generate a slug from heading text
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Generate a table of contents from blocks
pub fn generate_toc(blocks: &[Block]) -> Vec<TocEntry> {
    extract_headings(blocks)
        .into_iter()
        .map(|(level, text)| {
            let slug = slugify(&text);
            TocEntry { level, text, slug }
        })
        .collect()
}

/// Render TOC as a markdown list
pub fn render_toc(toc: &[TocEntry]) -> String {
    toc.iter()
        .map(|entry| {
            let indent = "  ".repeat((entry.level as usize).saturating_sub(1));
            format!("{}- [{}](#{})", indent, entry.text, entry.slug)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
