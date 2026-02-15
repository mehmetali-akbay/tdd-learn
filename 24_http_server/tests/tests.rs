use http_server::*;

// ===== Topic 1: Method & StatusCode =====

#[test]
fn test_method_parse() {
    assert_eq!(Method::parse("GET"), Some(Method::Get));
    assert_eq!(Method::parse("post"), Some(Method::Post));
    assert_eq!(Method::parse("PATCH"), None);
}

#[test]
fn test_method_display() {
    assert_eq!(format!("{}", Method::Get), "GET");
    assert_eq!(format!("{}", Method::Delete), "DELETE");
}

#[test]
fn test_status_code() {
    let ok = StatusCode::ok();
    assert_eq!(ok.code(), 200);
    assert_eq!(ok.reason(), "OK");
    assert!(ok.is_success());
    assert!(!ok.is_error());

    let nf = StatusCode::not_found();
    assert!(nf.is_error());
    assert!(!nf.is_success());
}

#[test]
fn test_status_display() {
    assert_eq!(format!("{}", StatusCode::ok()), "200 OK");
    assert_eq!(format!("{}", StatusCode::not_found()), "404 Not Found");
}

// ===== Topic 2: Headers =====

#[test]
fn test_headers_set_get() {
    let mut h = Headers::new();
    h.set("Content-Type", "text/html");
    assert_eq!(h.get("content-type"), Some("text/html")); // case insensitive
    assert_eq!(h.get("Content-Type"), Some("text/html"));
    assert!(h.has("content-type"));
}

#[test]
fn test_headers_overwrite() {
    let mut h = Headers::new();
    h.set("Host", "example.com");
    h.set("host", "other.com"); // overwrites
    assert_eq!(h.get("Host"), Some("other.com"));
    assert_eq!(h.len(), 1);
}

#[test]
fn test_headers_remove() {
    let mut h = Headers::new();
    h.set("X-Custom", "value");
    assert!(h.remove("x-custom"));
    assert!(!h.has("x-custom"));
    assert!(!h.remove("x-custom")); // already gone
}

#[test]
fn test_headers_content_helpers() {
    let mut h = Headers::new();
    h.set("Content-Type", "application/json");
    h.set("Content-Length", "42");
    assert_eq!(h.content_type(), Some("application/json"));
    assert_eq!(h.content_length(), Some(42));
}

#[test]
fn test_headers_to_http() {
    let mut h = Headers::new();
    h.set("Host", "example.com");
    let http = h.to_http();
    assert!(http.contains("host: example.com"));
}

// ===== Topic 3: Request Parsing =====

#[test]
fn test_parse_get_request() {
    let raw = "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
    let req = Request::parse(raw).unwrap();
    assert_eq!(req.method, Method::Get);
    assert_eq!(req.path, "/index.html");
    assert_eq!(req.version, "HTTP/1.1");
    assert!(req.is_get());
}

#[test]
fn test_parse_post_with_body() {
    let raw =
        "POST /api/data HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}";
    let req = Request::parse(raw).unwrap();
    assert!(req.is_post());
    assert_eq!(req.body, "{\"key\":\"value\"}");
    assert_eq!(req.headers.content_type(), Some("application/json"));
}

#[test]
fn test_parse_query_params() {
    let raw = "GET /search?q=rust&page=1 HTTP/1.1\r\n\r\n";
    let req = Request::parse(raw).unwrap();
    assert_eq!(req.path, "/search");
    assert_eq!(req.query_params.get("q"), Some(&"rust".to_string()));
    assert_eq!(req.query_params.get("page"), Some(&"1".to_string()));
}

#[test]
fn test_parse_invalid() {
    assert!(Request::parse("").is_err());
    assert!(Request::parse("INVALID").is_err());
}

#[test]
fn test_parse_query_string() {
    let params = parse_query_string("a=1&b=2&c=");
    assert_eq!(params.get("a"), Some(&"1".to_string()));
    assert_eq!(params.get("c"), Some(&"".to_string()));
}

// ===== Topic 4: Response Building =====

#[test]
fn test_response_ok() {
    let resp = Response::ok().with_body("Hello");
    assert_eq!(resp.status.code(), 200);
    assert_eq!(resp.body, "Hello");
    assert_eq!(resp.headers.content_length(), Some(5));
}

#[test]
fn test_response_json() {
    let resp = Response::ok().with_json("{\"ok\":true}");
    assert_eq!(resp.headers.content_type(), Some("application/json"));
}

#[test]
fn test_response_not_found() {
    let resp = Response::not_found();
    assert_eq!(resp.status.code(), 404);
}

#[test]
fn test_response_to_http() {
    let resp = Response::ok().with_body("Hi");
    let http = resp.to_http();
    assert!(http.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(http.ends_with("Hi"));
}

// ===== Topic 5: Router =====

#[test]
fn test_route_literal() {
    let route = Route::new(Method::Get, "/api/users");
    assert!(route.matches(&Method::Get, "/api/users").is_some());
    assert!(route.matches(&Method::Get, "/api/posts").is_none());
    assert!(route.matches(&Method::Post, "/api/users").is_none());
}

#[test]
fn test_route_params() {
    let route = Route::new(Method::Get, "/users/:id");
    let params = route.matches(&Method::Get, "/users/42").unwrap();
    assert_eq!(params.get("id"), Some(&"42".to_string()));
    assert!(route.matches(&Method::Get, "/users").is_none()); // too short
}

#[test]
fn test_route_wildcard() {
    let route = Route::new(Method::Get, "/static/*");
    assert!(route
        .matches(&Method::Get, "/static/css/main.css")
        .is_some());
    assert!(route.matches(&Method::Get, "/static/").is_some());
}

#[test]
fn test_router() {
    let mut router = Router::new();
    router.add_route(Method::Get, "/");
    router.add_route(Method::Get, "/users/:id");
    router.add_route(Method::Post, "/users");

    let (idx, params) = router.find_route(&Method::Get, "/users/5").unwrap();
    assert_eq!(idx, 1);
    assert_eq!(params.get("id"), Some(&"5".to_string()));

    let (idx, _) = router.find_route(&Method::Post, "/users").unwrap();
    assert_eq!(idx, 2);

    assert!(router.find_route(&Method::Delete, "/users").is_none());
}

// ===== Topic 6: URL Encoding =====

#[test]
fn test_url_decode() {
    assert_eq!(url_decode("hello%20world"), "hello world");
    assert_eq!(url_decode("a+b"), "a b");
    assert_eq!(url_decode("100%25"), "100%");
}

#[test]
fn test_url_encode() {
    assert_eq!(url_encode("hello world"), "hello+world");
    assert_eq!(url_encode("a&b=c"), "a%26b%3Dc");
    assert_eq!(url_encode("safe-text_here.ok"), "safe-text_here.ok");
}

#[test]
fn test_url_roundtrip() {
    let original = "search term with spaces & special=chars";
    assert_eq!(url_decode(&url_encode(original)), original);
}

// ===== Edge Cases =====

#[test]
fn test_method_parse_all() {
    assert_eq!(Method::parse("PUT"), Some(Method::Put));
    assert_eq!(Method::parse("DELETE"), Some(Method::Delete));
    assert_eq!(Method::parse("HEAD"), Some(Method::Head));
    assert_eq!(Method::parse("OPTIONS"), Some(Method::Options));
    assert_eq!(Method::parse(""), None);
}

#[test]
fn test_status_codes_all() {
    assert_eq!(StatusCode::created().code(), 201);
    assert_eq!(StatusCode::bad_request().code(), 400);
    assert_eq!(StatusCode::method_not_allowed().code(), 405);
    assert_eq!(StatusCode::internal_error().code(), 500);
    assert!(StatusCode::internal_error().is_error());
    assert!(StatusCode::created().is_success());
}

#[test]
fn test_headers_empty() {
    let h = Headers::new();
    assert!(h.is_empty());
    assert_eq!(h.len(), 0);
    assert_eq!(h.get("anything"), None);
    assert!(!h.has("anything"));
}

#[test]
fn test_headers_no_content_length() {
    let h = Headers::new();
    assert_eq!(h.content_length(), None);
    assert_eq!(h.content_type(), None);
}

#[test]
fn test_request_is_methods() {
    let get = Request::parse("GET / HTTP/1.1\r\n\r\n").unwrap();
    assert!(get.is_get());
    assert!(!get.is_post());

    let post = Request::parse("POST / HTTP/1.1\r\n\r\n").unwrap();
    assert!(post.is_post());
    assert!(!post.is_get());
}

#[test]
fn test_response_html() {
    let resp = Response::ok().with_html("<h1>Hello</h1>");
    assert_eq!(resp.headers.content_type(), Some("text/html"));
    assert_eq!(resp.body, "<h1>Hello</h1>");
}

#[test]
fn test_response_with_header() {
    let resp = Response::ok()
        .with_header("X-Custom", "value")
        .with_body("test");
    assert_eq!(resp.headers.get("X-Custom"), Some("value"));
}

#[test]
fn test_router_empty() {
    let router = Router::new();
    assert_eq!(router.route_count(), 0);
    assert!(router.find_route(&Method::Get, "/").is_none());
}

#[test]
fn test_router_count() {
    let mut router = Router::new();
    router.add_route(Method::Get, "/a");
    router.add_route(Method::Post, "/b");
    assert_eq!(router.route_count(), 2);
}

#[test]
fn test_url_encode_special() {
    assert_eq!(url_encode("100%"), "100%25");
    assert_eq!(url_encode("a/b"), "a%2Fb");
}

#[test]
fn test_url_decode_plain() {
    assert_eq!(url_decode("plain"), "plain");
    assert_eq!(url_decode(""), "");
}
