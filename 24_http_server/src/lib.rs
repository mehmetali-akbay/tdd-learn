// ============================================
// Level 6, Project 2: Mini HTTP Server (Parser)
// Learn: HTTP protocol, request/response parsing, routing
// ============================================

use std::collections::HashMap;
use std::fmt;

// ============================================
// Topic 1: HTTP Method & Status Code
// Learn: Enums for protocol elements
// ============================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
}

impl Method {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "DELETE" => Some(Method::Delete),
            "HEAD" => Some(Method::Head),
            "OPTIONS" => Some(Method::Options),
            _ => None,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Head => write!(f, "HEAD"),
            Method::Options => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusCode(pub u16, pub String);

impl StatusCode {
    pub fn ok() -> Self {
        StatusCode(200, "OK".to_string())
    }
    pub fn created() -> Self {
        StatusCode(201, "Created".to_string())
    }
    pub fn bad_request() -> Self {
        StatusCode(400, "Bad Request".to_string())
    }
    pub fn not_found() -> Self {
        StatusCode(404, "Not Found".to_string())
    }
    pub fn method_not_allowed() -> Self {
        StatusCode(405, "Method Not Allowed".to_string())
    }
    pub fn internal_error() -> Self {
        StatusCode(500, "Internal Server Error".to_string())
    }

    pub fn code(&self) -> u16 {
        self.0
    }

    pub fn reason(&self) -> &str {
        &self.1
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.0)
    }

    pub fn is_error(&self) -> bool {
        self.0 >= 400
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

// ============================================
// Topic 2: HTTP Headers
// Learn: Case-insensitive header map
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Headers {
    entries: Vec<(String, String)>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: &str) {
        let lower = name.to_lowercase();
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &lower) {
            entry.1 = value.to_string();
        } else {
            self.entries.push((lower, value.to_string()));
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        let lower = name.to_lowercase();
        self.entries
            .iter()
            .find(|(k, _)| k == &lower)
            .map(|(_, v)| v.as_str())
    }

    pub fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let lower = name.to_lowercase();
        let before = self.entries.len();
        self.entries.retain(|(k, _)| k != &lower);
        self.entries.len() < before
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.get("content-type")
    }

    pub fn content_length(&self) -> Option<usize> {
        self.get("content-length")?.parse().ok()
    }

    /// Serialize headers to HTTP format
    pub fn to_http(&self) -> String {
        self.entries
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 3: HTTP Request — Parsing
// Learn: Parsing raw HTTP text into structured data
// ============================================

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: Headers,
    pub body: String,
    pub query_params: HashMap<String, String>,
}

impl Request {
    /// Parse a raw HTTP request string
    pub fn parse(raw: &str) -> Result<Self, String> {
        let mut sections = raw.splitn(2, "\r\n\r\n");
        let header_section = sections.next().ok_or("empty request")?;
        let body = sections.next().unwrap_or("").to_string();

        let mut lines = header_section.lines();
        let request_line = lines.next().ok_or("missing request line")?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("invalid request line".to_string());
        }

        let method =
            Method::parse(parts[0]).ok_or_else(|| format!("unknown method: {}", parts[0]))?;
        let full_path = parts[1].to_string();
        let version = parts[2].to_string();

        // Parse path and query string
        let (path, query_params) = if let Some(q_pos) = full_path.find('?') {
            let path = full_path[..q_pos].to_string();
            let query_str = &full_path[q_pos + 1..];
            let params = parse_query_string(query_str);
            (path, params)
        } else {
            (full_path, HashMap::new())
        };

        let mut headers = Headers::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                headers.set(name, value);
            }
        }

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
            query_params,
        })
    }

    pub fn is_get(&self) -> bool {
        self.method == Method::Get
    }

    pub fn is_post(&self) -> bool {
        self.method == Method::Post
    }
}

/// Parse a query string like "key1=val1&key2=val2"
pub fn parse_query_string(qs: &str) -> HashMap<String, String> {
    qs.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts.next().unwrap_or("").to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect()
}

// ============================================
// Topic 4: HTTP Response — Building
// Learn: Constructing HTTP responses
// ============================================

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: String,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Response {
            status,
            headers: Headers::new(),
            body: String::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(StatusCode::ok())
    }

    pub fn not_found() -> Self {
        let mut resp = Self::new(StatusCode::not_found());
        resp.body = "404 Not Found".to_string();
        resp
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self.headers.set("content-length", &body.len().to_string());
        self
    }

    pub fn with_json(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self.headers.set("content-type", "application/json");
        self.headers.set("content-length", &body.len().to_string());
        self
    }

    pub fn with_html(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self.headers.set("content-type", "text/html");
        self.headers.set("content-length", &body.len().to_string());
        self
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.set(name, value);
        self
    }

    /// Serialize to raw HTTP response
    pub fn to_http(&self) -> String {
        let mut result = format!("HTTP/1.1 {}\r\n", self.status);
        if !self.headers.is_empty() {
            result.push_str(&self.headers.to_http());
            result.push_str("\r\n");
        }
        result.push_str("\r\n");
        result.push_str(&self.body);
        result
    }
}

// ============================================
// Topic 5: Router — Path Matching
// Learn: Pattern matching for routes, path parameters
// ============================================

/// A route pattern that can match paths
#[derive(Debug, Clone)]
pub struct Route {
    pub method: Method,
    pub pattern: String,
    segments: Vec<RouteSegment>,
}

#[derive(Debug, Clone)]
enum RouteSegment {
    Literal(String),
    Param(String),
    Wildcard,
}

impl Route {
    pub fn new(method: Method, pattern: &str) -> Self {
        let segments = pattern
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                if let Some(param) = s.strip_prefix(':') {
                    RouteSegment::Param(param.to_string())
                } else if s == "*" {
                    RouteSegment::Wildcard
                } else {
                    RouteSegment::Literal(s.to_string())
                }
            })
            .collect();
        Route {
            method,
            pattern: pattern.to_string(),
            segments,
        }
    }

    /// Try to match a path, returning extracted parameters
    pub fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if &self.method != method {
            return None;
        }

        let path_parts: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Check for wildcard at end
        let has_wildcard = self
            .segments
            .last()
            .map(|s| matches!(s, RouteSegment::Wildcard))
            .unwrap_or(false);

        if !has_wildcard && path_parts.len() != self.segments.len() {
            return None;
        }

        if has_wildcard && path_parts.len() < self.segments.len() - 1 {
            return None;
        }

        let mut params = HashMap::new();
        for (i, seg) in self.segments.iter().enumerate() {
            match seg {
                RouteSegment::Literal(expected) => {
                    if i >= path_parts.len() || path_parts[i] != expected.as_str() {
                        return None;
                    }
                }
                RouteSegment::Param(name) => {
                    if i >= path_parts.len() {
                        return None;
                    }
                    params.insert(name.clone(), path_parts[i].to_string());
                }
                RouteSegment::Wildcard => {
                    return Some(params); // match everything after
                }
            }
        }
        Some(params)
    }
}

/// A simple router that finds matching routes
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, method: Method, pattern: &str) {
        self.routes.push(Route::new(method, pattern));
    }

    /// Find the first matching route, returning (route_index, params)
    pub fn find_route(
        &self,
        method: &Method,
        path: &str,
    ) -> Option<(usize, HashMap<String, String>)> {
        for (i, route) in self.routes.iter().enumerate() {
            if let Some(params) = route.matches(method, path) {
                return Some((i, params));
            }
        }
        None
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 6: URL Encoding/Decoding
// Learn: Percent-encoding, URL parsing
// ============================================

/// Decode a percent-encoded string
pub fn url_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
            result.push('%');
            result.push_str(&hex);
        } else if ch == '+' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    result
}

/// Encode a string with percent-encoding
pub fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(ch),
            ' ' => result.push('+'),
            _ => {
                for byte in ch.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}
