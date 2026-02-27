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
    todo!()
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
        todo!()
    }

    pub fn is_heading(&self) -> bool {
        todo!()
    }

    pub fn is_paragraph(&self) -> bool {
        todo!()
    }

    pub fn is_code_block(&self) -> bool {
        todo!()
    }
}

// ============================================
// Topic 3: Block Parser
// Learn: Line-based parsing, state machine
// ============================================

/// Parse markdown text into a list of blocks
pub fn parse_blocks(input: &str) -> Vec<Block> {
    todo!()
}

// ============================================
// Topic 4: HTML Renderer
// Learn: AST to HTML conversion, escaping
// ============================================

/// Escape HTML special characters
pub fn escape_html(s: &str) -> String {
    todo!()
}

/// Render inline elements to HTML
pub fn render_inline(elements: &[Inline]) -> String {
    todo!()
}

/// Render a block to HTML
pub fn render_block(block: &Block) -> String {
    todo!()
}

/// Render a full markdown document to HTML
pub fn render(input: &str) -> String {
    todo!()
}

// ============================================
// Topic 5: Document Structure — Extraction
// Learn: Extracting metadata from parsed documents
// ============================================

/// Extract all headings from a document
pub fn extract_headings(blocks: &[Block]) -> Vec<(u8, String)> {
    todo!()
}

/// Extract all links from a document
pub fn extract_links(blocks: &[Block]) -> Vec<(String, String)> {
    todo!()
}

fn extract_links_from_inline(inlines: &[Inline], links: &mut Vec<(String, String)>) {
    todo!()
}

/// Count words in a document (text only)
pub fn word_count(blocks: &[Block]) -> usize {
    todo!()
}

fn count_inline_words(inlines: &[Inline]) -> usize {
    todo!()
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
    todo!()
}

/// Generate a table of contents from blocks
pub fn generate_toc(blocks: &[Block]) -> Vec<TocEntry> {
    todo!()
}

/// Render TOC as a markdown list
pub fn render_toc(toc: &[TocEntry]) -> String {
    todo!()
}
