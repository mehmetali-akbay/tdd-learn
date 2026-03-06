use trait_objects::*;

#[test]
fn test_circle_draw() {
    assert!(
        Circle { radius: 5.0 }.draw().contains("circle")
            || Circle { radius: 5.0 }.draw().contains("Circle")
    );
}
#[test]
fn test_square_draw() {
    assert!(
        Square { side: 3.0 }.draw().contains("square")
            || Square { side: 3.0 }.draw().contains("Square")
    );
}
#[test]
fn test_create_drawable() {
    assert_eq!(create_drawable("circle").name(), "circle");
}
#[test]
fn test_draw_all() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    assert_eq!(draw_all(&items).len(), 2);
}
#[test]
fn test_find_by_name() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    assert!(find_by_name(&items, "circle").is_some());
    assert!(find_by_name(&items, "triangle").is_none());
}
#[test]
fn test_canvas() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.add(Box::new(Text {
        content: "hello".into(),
    }));
    assert_eq!(c.count(), 2);
    assert_eq!(c.render().len(), 2);
}
#[test]
fn test_post_draft_no_content() {
    let p = Post::new();
    assert_eq!(p.content(), "");
    assert_eq!(p.state_name(), "draft");
}
#[test]
fn test_post_workflow() {
    let mut p = Post::new();
    p.add_text("Hello World");
    assert_eq!(p.content(), "");
    p.request_review();
    assert_eq!(p.state_name(), "pending_review");
    assert_eq!(p.content(), "");
    p.approve();
    assert_eq!(p.state_name(), "published");
    assert_eq!(p.content(), "Hello World");
}
#[test]
fn test_post_reject() {
    let mut p = Post::new();
    p.add_text("Draft content");
    p.request_review();
    p.reject();
    assert_eq!(p.state_name(), "draft");
    assert_eq!(p.content(), "");
}
#[test]
fn test_post_approve_without_review() {
    let mut p = Post::new();
    p.add_text("Content");
    p.approve();
    assert_eq!(p.state_name(), "draft");
}
#[test]
fn test_bubble_sort() {
    let s = Sorter::new(Box::new(BubbleSort));
    let mut d = vec![3, 1, 4, 1, 5];
    s.sort(&mut d);
    assert_eq!(d, vec![1, 1, 3, 4, 5]);
}
#[test]
fn test_insertion_sort() {
    let s = Sorter::new(Box::new(InsertionSort));
    let mut d = vec![5, 2, 8, 1];
    s.sort(&mut d);
    assert_eq!(d, vec![1, 2, 5, 8]);
}
#[test]
fn test_quick_sort() {
    let s = Sorter::new(Box::new(QuickSort));
    let mut d = vec![9, 3, 7, 1, 5];
    s.sort(&mut d);
    assert_eq!(d, vec![1, 3, 5, 7, 9]);
}
#[test]
fn test_change_strategy() {
    let mut s = Sorter::new(Box::new(BubbleSort));
    assert_eq!(s.strategy_name(), "bubble_sort");
    s.set_strategy(Box::new(QuickSort));
    assert_eq!(s.strategy_name(), "quick_sort");
}
#[test]
fn test_anybox_i32() {
    let b = AnyBox::new(42i32);
    assert_eq!(b.get::<i32>(), Some(&42));
    assert_eq!(b.get::<String>(), None);
}
#[test]
fn test_anybox_string() {
    let b = AnyBox::new("hello".to_string());
    assert_eq!(b.get::<String>(), Some(&"hello".to_string()));
    assert!(b.is::<String>());
    assert!(!b.is::<i32>());
}
#[test]
fn test_typemap() {
    let mut m = TypeMap::new();
    m.insert(42i32);
    m.insert("hello".to_string());
    assert_eq!(m.get::<i32>(), Some(&42));
    assert_eq!(m.get::<String>(), Some(&"hello".to_string()));
    assert_eq!(m.get::<f64>(), None);
    assert_eq!(m.count(), 2);
}
#[test]
fn test_format_static() {
    assert_eq!(format_static("hello", &UpperFormatter), "HELLO");
}
#[test]
fn test_format_dynamic() {
    let f: &dyn Formatter = &LowerFormatter;
    assert_eq!(format_dynamic("HELLO", f), "hello");
}
#[test]
fn test_title_formatter() {
    assert_eq!(TitleFormatter.format("hello world"), "Hello World");
}
#[test]
fn test_format_all() {
    let f: Vec<&dyn Formatter> = vec![&UpperFormatter, &LowerFormatter];
    assert_eq!(format_all("Hello", &f), vec!["HELLO", "hello"]);
}

// ===== Edge Cases =====

#[test]
fn test_create_drawable_square() {
    assert_eq!(create_drawable("square").name(), "square");
}

#[test]
fn test_create_drawable_text() {
    assert_eq!(create_drawable("text").name(), "text");
}

#[test]
fn test_canvas_empty() {
    let c = Canvas::new();
    assert_eq!(c.count(), 0);
    assert!(c.render().is_empty());
}

#[test]
fn test_draw_all_empty() {
    let items: Vec<Box<dyn Drawable>> = vec![];
    assert!(draw_all(&items).is_empty());
}

#[test]
fn test_sort_empty() {
    let s = Sorter::new(Box::new(BubbleSort));
    let mut d: Vec<i32> = vec![];
    s.sort(&mut d);
    assert!(d.is_empty());
}

#[test]
fn test_sort_already_sorted() {
    let s = Sorter::new(Box::new(QuickSort));
    let mut d = vec![1, 2, 3, 4, 5];
    s.sort(&mut d);
    assert_eq!(d, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_typemap_multiple_types() {
    let mut m = TypeMap::new();
    m.insert(42i32);
    m.insert(3.14f64);
    m.insert("hello".to_string());
    assert_eq!(m.get::<i32>(), Some(&42));
    assert_eq!(m.get::<f64>(), Some(&3.14));
    assert_eq!(m.count(), 3);
}
