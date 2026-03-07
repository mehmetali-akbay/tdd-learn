use trait_objects::*;

// ============================================
// Topic 1: Trait Objects Basics
// ============================================

#[test]
fn test_circle_draw() {
    let c = Circle { radius: 5.0 };
    assert!(c.draw().contains("circle") || c.draw().contains("Circle"));
    assert!(c.draw().contains("5"));
}

#[test]
fn test_circle_name() {
    assert_eq!(Circle { radius: 1.0 }.name(), "circle");
}

#[test]
fn test_circle_area() {
    let c = Circle { radius: 2.0 };
    let expected = std::f64::consts::PI * 4.0;
    assert!((c.area() - expected).abs() < 1e-10);
}

#[test]
fn test_square_draw() {
    let s = Square { side: 3.0 };
    assert!(s.draw().contains("square") || s.draw().contains("Square"));
    assert!(s.draw().contains("3"));
}

#[test]
fn test_square_name() {
    assert_eq!(Square { side: 1.0 }.name(), "square");
}

#[test]
fn test_square_area() {
    assert!((Square { side: 4.0 }.area() - 16.0).abs() < 1e-10);
}

#[test]
fn test_text_draw() {
    let t = Text {
        content: "hello".into(),
    };
    assert!(t.draw().contains("hello"));
}

#[test]
fn test_text_name() {
    assert_eq!(
        Text {
            content: "x".into()
        }
        .name(),
        "text"
    );
}

#[test]
fn test_text_area_zero() {
    assert_eq!(
        Text {
            content: "a".into()
        }
        .area(),
        0.0
    );
}

#[test]
fn test_triangle_draw() {
    let t = Triangle {
        base: 3.0,
        height: 4.0,
    };
    assert!(t.draw().contains("triangle") || t.draw().contains("Triangle"));
}

#[test]
fn test_triangle_name() {
    assert_eq!(
        Triangle {
            base: 1.0,
            height: 1.0
        }
        .name(),
        "triangle"
    );
}

#[test]
fn test_triangle_area() {
    let t = Triangle {
        base: 6.0,
        height: 4.0,
    };
    assert!((t.area() - 12.0).abs() < 1e-10);
}

#[test]
fn test_create_drawable_circle() {
    let d = create_drawable("circle");
    assert_eq!(d.name(), "circle");
}

#[test]
fn test_create_drawable_square() {
    let d = create_drawable("square");
    assert_eq!(d.name(), "square");
}

#[test]
fn test_create_drawable_triangle() {
    let d = create_drawable("triangle");
    assert_eq!(d.name(), "triangle");
}

#[test]
fn test_create_drawable_unknown_becomes_text() {
    let d = create_drawable("anything");
    assert_eq!(d.name(), "text");
}

#[test]
fn test_total_area() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square { side: 2.0 }),  // 4.0
        Box::new(Square { side: 3.0 }),  // 9.0
    ];
    assert!((total_area(&items) - 13.0).abs() < 1e-10);
}

#[test]
fn test_total_area_empty() {
    let items: Vec<Box<dyn Drawable>> = vec![];
    assert_eq!(total_area(&items), 0.0);
}

#[test]
fn test_largest_by_area() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 10.0 }),
        Box::new(Triangle { base: 2.0, height: 3.0 }),
    ];
    assert_eq!(largest_by_area(&items), Some("square".to_string()));
}

#[test]
fn test_largest_by_area_empty() {
    let items: Vec<Box<dyn Drawable>> = vec![];
    assert_eq!(largest_by_area(&items), None);
}

// ============================================
// Topic 2: Heterogeneous Collections
// ============================================

#[test]
fn test_draw_all() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    let results = draw_all(&items);
    assert_eq!(results.len(), 2);
    assert!(results[0].contains("circle") || results[0].contains("Circle"));
}

#[test]
fn test_draw_all_empty() {
    let items: Vec<Box<dyn Drawable>> = vec![];
    assert!(draw_all(&items).is_empty());
}

#[test]
fn test_find_by_name_found() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    assert!(find_by_name(&items, "circle").is_some());
}

#[test]
fn test_find_by_name_not_found() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
    ];
    assert!(find_by_name(&items, "triangle").is_none());
}

#[test]
fn test_get_all_names() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Text { content: "hi".into() }),
    ];
    let names = get_all_names(&items);
    assert_eq!(names, vec!["circle", "square", "text"]);
}

#[test]
fn test_filter_by_min_area() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square { side: 2.0 }),   // area 4
        Box::new(Square { side: 1.0 }),   // area 1
        Box::new(Text { content: "x".into() }), // area 0
    ];
    let result = filter_by_min_area(&items, 2.0);
    assert_eq!(result, vec!["square".to_string()]);
}

#[test]
fn test_filter_by_min_area_all() {
    let items: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square { side: 5.0 }),   // area 25
        Box::new(Square { side: 3.0 }),   // area 9
    ];
    let result = filter_by_min_area(&items, 1.0);
    assert_eq!(result.len(), 2);
}

#[test]
fn test_canvas_new_empty() {
    let c = Canvas::new();
    assert_eq!(c.count(), 0);
    assert!(c.is_empty());
    assert!(c.render().is_empty());
}

#[test]
fn test_canvas_add_and_count() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.add(Box::new(Square { side: 2.0 }));
    assert_eq!(c.count(), 2);
    assert!(!c.is_empty());
}

#[test]
fn test_canvas_render() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.add(Box::new(Text { content: "hello".into() }));
    let rendered = c.render();
    assert_eq!(rendered.len(), 2);
}

#[test]
fn test_canvas_names() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.add(Box::new(Square { side: 2.0 }));
    assert_eq!(c.names(), vec!["circle", "square"]);
}

#[test]
fn test_canvas_total_area() {
    let mut c = Canvas::new();
    c.add(Box::new(Square { side: 3.0 }));
    c.add(Box::new(Square { side: 4.0 }));
    assert!((c.total_area() - 25.0).abs() < 1e-10);
}

#[test]
fn test_canvas_clear() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.clear();
    assert!(c.is_empty());
}

#[test]
fn test_canvas_remove_by_name() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    c.add(Box::new(Square { side: 2.0 }));
    assert!(c.remove_by_name("circle"));
    assert_eq!(c.count(), 1);
    assert_eq!(c.names(), vec!["square"]);
}

#[test]
fn test_canvas_remove_by_name_not_found() {
    let mut c = Canvas::new();
    c.add(Box::new(Circle { radius: 1.0 }));
    assert!(!c.remove_by_name("triangle"));
    assert_eq!(c.count(), 1);
}

#[test]
fn test_canvas_default() {
    let c = Canvas::default();
    assert!(c.is_empty());
}

// ============================================
// Topic 3: State Pattern
// ============================================

#[test]
fn test_post_new_is_draft() {
    let p = Post::new();
    assert_eq!(p.state_name(), "draft");
    assert_eq!(p.content(), "");
    assert!(!p.is_published());
}

#[test]
fn test_post_add_text() {
    let mut p = Post::new();
    p.add_text("Hello ");
    p.add_text("World");
    // Content not visible in draft
    assert_eq!(p.content(), "");
}

#[test]
fn test_post_full_workflow() {
    let mut p = Post::new();
    p.add_text("Hello World");
    assert_eq!(p.content(), "");
    p.request_review();
    assert_eq!(p.state_name(), "pending_review");
    assert_eq!(p.content(), "");
    p.approve();
    assert_eq!(p.state_name(), "published");
    assert_eq!(p.content(), "Hello World");
    assert!(p.is_published());
}

#[test]
fn test_post_reject_goes_back_to_draft() {
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
fn test_post_double_review() {
    let mut p = Post::new();
    p.add_text("Text");
    p.request_review();
    p.request_review(); // stays in pending_review
    assert_eq!(p.state_name(), "pending_review");
}

#[test]
fn test_post_published_stays_published() {
    let mut p = Post::new();
    p.add_text("Text");
    p.request_review();
    p.approve();
    p.approve();         // stays published
    p.request_review();  // stays published
    p.reject();          // stays published
    assert_eq!(p.state_name(), "published");
    assert!(p.is_published());
}

#[test]
fn test_post_default() {
    let p = Post::default();
    assert_eq!(p.state_name(), "draft");
}

#[test]
fn test_post_reject_then_re_review() {
    let mut p = Post::new();
    p.add_text("Content");
    p.request_review();
    p.reject();
    assert_eq!(p.state_name(), "draft");
    p.request_review();
    assert_eq!(p.state_name(), "pending_review");
    p.approve();
    assert_eq!(p.content(), "Content");
}

// ============================================
// Topic 4: Strategy Pattern
// ============================================

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
fn test_selection_sort() {
    let s = Sorter::new(Box::new(SelectionSort));
    let mut d = vec![4, 2, 7, 1, 3];
    s.sort(&mut d);
    assert_eq!(d, vec![1, 2, 3, 4, 7]);
}

#[test]
fn test_change_strategy() {
    let mut s = Sorter::new(Box::new(BubbleSort));
    assert_eq!(s.strategy_name(), "bubble_sort");
    s.set_strategy(Box::new(QuickSort));
    assert_eq!(s.strategy_name(), "quick_sort");
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
fn test_sort_single_element() {
    let s = Sorter::new(Box::new(InsertionSort));
    let mut d = vec![42];
    s.sort(&mut d);
    assert_eq!(d, vec![42]);
}

#[test]
fn test_sort_with_all() {
    let data = vec![3, 1, 2];
    let strategies: Vec<&dyn SortStrategy> = vec![&BubbleSort, &InsertionSort, &QuickSort];
    let results = sort_with_all(&data, &strategies);
    assert_eq!(results.len(), 3);
    for r in &results {
        assert_eq!(*r, vec![1, 2, 3]);
    }
}

#[test]
fn test_sort_with_all_empty_strategies() {
    let data = vec![3, 1, 2];
    let strategies: Vec<&dyn SortStrategy> = vec![];
    assert!(sort_with_all(&data, &strategies).is_empty());
}

#[test]
fn test_strategy_names() {
    assert_eq!(BubbleSort.name(), "bubble_sort");
    assert_eq!(InsertionSort.name(), "insertion_sort");
    assert_eq!(QuickSort.name(), "quick_sort");
    assert_eq!(SelectionSort.name(), "selection_sort");
}

// ============================================
// Topic 5: Any & Downcasting
// ============================================

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
fn test_anybox_f64() {
    let b = AnyBox::new(3.14f64);
    assert!(b.is::<f64>());
    assert_eq!(b.get::<f64>(), Some(&3.14));
}

#[test]
fn test_anybox_replace() {
    let mut b = AnyBox::new(42i32);
    let old = b.replace("hello".to_string());
    assert!(old.is::<i32>());
    assert!(b.is::<String>());
    assert_eq!(b.get::<String>(), Some(&"hello".to_string()));
}

#[test]
fn test_typemap_basic() {
    let mut m = TypeMap::new();
    m.insert(42i32);
    m.insert("hello".to_string());
    assert_eq!(m.get::<i32>(), Some(&42));
    assert_eq!(m.get::<String>(), Some(&"hello".to_string()));
    assert_eq!(m.get::<f64>(), None);
    assert_eq!(m.count(), 2);
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

#[test]
fn test_typemap_get_all() {
    let mut m = TypeMap::new();
    m.insert(1i32);
    m.insert(2i32);
    m.insert(3i32);
    m.insert("hi".to_string());
    let all_ints = m.get_all::<i32>();
    assert_eq!(all_ints.len(), 3);
    assert_eq!(*all_ints[0], 1);
    assert_eq!(*all_ints[1], 2);
    assert_eq!(*all_ints[2], 3);
}

#[test]
fn test_typemap_count_of() {
    let mut m = TypeMap::new();
    m.insert(1i32);
    m.insert(2i32);
    m.insert("hi".to_string());
    assert_eq!(m.count_of::<i32>(), 2);
    assert_eq!(m.count_of::<String>(), 1);
    assert_eq!(m.count_of::<f64>(), 0);
}

#[test]
fn test_typemap_remove_all_of() {
    let mut m = TypeMap::new();
    m.insert(1i32);
    m.insert(2i32);
    m.insert("hi".to_string());
    let removed = m.remove_all_of::<i32>();
    assert_eq!(removed, 2);
    assert_eq!(m.count(), 1);
    assert_eq!(m.get::<i32>(), None);
}

#[test]
fn test_typemap_is_empty() {
    let m = TypeMap::new();
    assert!(m.is_empty());
    let mut m2 = TypeMap::new();
    m2.insert(1i32);
    assert!(!m2.is_empty());
}

#[test]
fn test_typemap_default() {
    let m = TypeMap::default();
    assert!(m.is_empty());
}

// ============================================
// Topic 6: Static vs Dynamic Dispatch
// ============================================

#[test]
fn test_format_static_upper() {
    assert_eq!(format_static("hello", &UpperFormatter), "HELLO");
}

#[test]
fn test_format_static_lower() {
    assert_eq!(format_static("HELLO", &LowerFormatter), "hello");
}

#[test]
fn test_format_dynamic_upper() {
    let f: &dyn Formatter = &UpperFormatter;
    assert_eq!(format_dynamic("hello", f), "HELLO");
}

#[test]
fn test_format_dynamic_lower() {
    let f: &dyn Formatter = &LowerFormatter;
    assert_eq!(format_dynamic("HELLO", f), "hello");
}

#[test]
fn test_title_formatter() {
    assert_eq!(TitleFormatter.format("hello world"), "Hello World");
}

#[test]
fn test_title_formatter_single_word() {
    assert_eq!(TitleFormatter.format("hello"), "Hello");
}

#[test]
fn test_trim_formatter() {
    assert_eq!(TrimFormatter.format("  hello  "), "hello");
}

#[test]
fn test_reverse_formatter() {
    assert_eq!(ReverseFormatter.format("hello"), "olleh");
}

#[test]
fn test_format_all() {
    let f: Vec<&dyn Formatter> = vec![&UpperFormatter, &LowerFormatter];
    assert_eq!(format_all("Hello", &f), vec!["HELLO", "hello"]);
}

#[test]
fn test_format_all_empty() {
    let f: Vec<&dyn Formatter> = vec![];
    assert!(format_all("Hello", &f).is_empty());
}

#[test]
fn test_format_chain() {
    // trim then uppercase
    let formatters: Vec<&dyn Formatter> = vec![&TrimFormatter, &UpperFormatter];
    assert_eq!(format_chain("  hello  ", &formatters), "HELLO");
}

#[test]
fn test_format_chain_reverse_then_upper() {
    let formatters: Vec<&dyn Formatter> = vec![&ReverseFormatter, &UpperFormatter];
    assert_eq!(format_chain("hello", &formatters), "OLLEH");
}

#[test]
fn test_format_chain_empty() {
    let formatters: Vec<&dyn Formatter> = vec![];
    assert_eq!(format_chain("hello", &formatters), "hello");
}

// ============================================
// Topic 7: Observer Pattern
// ============================================

#[test]
fn test_logger_observer() {
    let mut logger = Logger::new("test");
    logger.on_event("event1");
    logger.on_event("event2");
    assert_eq!(logger.logs.len(), 2);
    assert!(logger.logs[0].contains("test"));
    assert!(logger.logs[0].contains("event1"));
}

#[test]
fn test_counter_observer() {
    let mut counter = Counter::new("cnt");
    counter.on_event("a");
    counter.on_event("b");
    counter.on_event("c");
    assert_eq!(counter.count, 3);
}

#[test]
fn test_eventbus_subscribe_and_emit() {
    let mut bus = EventBus::new();
    bus.subscribe(Box::new(Logger::new("log1")));
    bus.subscribe(Box::new(Counter::new("cnt1")));
    assert_eq!(bus.observer_count(), 2);
    bus.emit("test_event");
    // Observers are stored as trait objects, can't directly inspect
    // but observer_count and names should work
    assert_eq!(bus.observer_names(), vec!["log1", "cnt1"]);
}

#[test]
fn test_eventbus_empty() {
    let bus = EventBus::new();
    assert_eq!(bus.observer_count(), 0);
    assert!(bus.observer_names().is_empty());
}

#[test]
fn test_eventbus_unsubscribe() {
    let mut bus = EventBus::new();
    bus.subscribe(Box::new(Logger::new("a")));
    bus.subscribe(Box::new(Logger::new("b")));
    assert!(bus.unsubscribe("a"));
    assert_eq!(bus.observer_count(), 1);
    assert_eq!(bus.observer_names(), vec!["b"]);
}

#[test]
fn test_eventbus_unsubscribe_not_found() {
    let mut bus = EventBus::new();
    bus.subscribe(Box::new(Logger::new("a")));
    assert!(!bus.unsubscribe("z"));
    assert_eq!(bus.observer_count(), 1);
}

#[test]
fn test_eventbus_default() {
    let bus = EventBus::default();
    assert_eq!(bus.observer_count(), 0);
}

// ============================================
// Topic 8: Command Pattern
// ============================================

#[test]
fn test_add_command() {
    let mut state = vec![];
    let cmd = AddCommand::new("apple");
    cmd.execute(&mut state);
    assert_eq!(state, vec!["apple"]);
    cmd.undo(&mut state);
    assert!(state.is_empty());
}

#[test]
fn test_remove_command() {
    let mut state = vec!["apple".to_string(), "banana".to_string()];
    let cmd = RemoveCommand::new("apple");
    cmd.execute(&mut state);
    assert_eq!(state, vec!["banana"]);
    cmd.undo(&mut state);
    assert_eq!(state, vec!["banana", "apple"]);
}

#[test]
fn test_clear_command() {
    let mut state = vec!["a".to_string(), "b".to_string()];
    let cmd = ClearCommand;
    cmd.execute(&mut state);
    assert!(state.is_empty());
}

#[test]
fn test_command_descriptions() {
    assert_eq!(AddCommand::new("x").description(), "Add 'x'");
    assert_eq!(RemoveCommand::new("y").description(), "Remove 'y'");
    assert_eq!(ClearCommand.description(), "Clear all");
}

#[test]
fn test_command_history_execute() {
    let mut h = CommandHistory::new();
    h.execute(Box::new(AddCommand::new("a")));
    h.execute(Box::new(AddCommand::new("b")));
    assert_eq!(h.state(), &["a", "b"]);
    assert_eq!(h.history_len(), 2);
}

#[test]
fn test_command_history_undo() {
    let mut h = CommandHistory::new();
    h.execute(Box::new(AddCommand::new("a")));
    h.execute(Box::new(AddCommand::new("b")));
    assert!(h.undo_last());
    assert_eq!(h.state(), &["a"]);
    assert_eq!(h.history_len(), 1);
}

#[test]
fn test_command_history_undo_empty() {
    let mut h = CommandHistory::new();
    assert!(!h.undo_last());
}

#[test]
fn test_command_history_descriptions() {
    let mut h = CommandHistory::new();
    h.execute(Box::new(AddCommand::new("x")));
    h.execute(Box::new(RemoveCommand::new("y")));
    let descs = h.history_descriptions();
    assert_eq!(descs, vec!["Add 'x'", "Remove 'y'"]);
}

#[test]
fn test_command_history_default() {
    let h = CommandHistory::default();
    assert!(h.state().is_empty());
    assert_eq!(h.history_len(), 0);
}

#[test]
fn test_command_history_complex_workflow() {
    let mut h = CommandHistory::new();
    h.execute(Box::new(AddCommand::new("a")));
    h.execute(Box::new(AddCommand::new("b")));
    h.execute(Box::new(AddCommand::new("c")));
    h.execute(Box::new(RemoveCommand::new("b")));
    assert_eq!(h.state(), &["a", "c"]);
    h.undo_last(); // undo remove "b" -> pushes "b" back
    assert_eq!(h.state(), &["a", "c", "b"]);
    h.undo_last(); // undo add "c"
    assert_eq!(h.state(), &["a", "b"]);
}

// ============================================
// Topic 9: Trait Object Composition & Multi-trait
// ============================================

#[test]
fn test_book_product() {
    let b = Book {
        id: 1,
        title: "Rust".into(),
        cost: 29.99,
    };
    assert_eq!(b.describe(), "Book: Rust");
    assert!((b.price() - 29.99).abs() < 1e-10);
    assert_eq!(b.product_id(), 1);
}

#[test]
fn test_gadget_product() {
    let g = Gadget {
        id: 2,
        name: "Widget".into(),
        cost: 9.99,
    };
    assert_eq!(g.describe(), "Gadget: Widget");
    assert!((g.price() - 9.99).abs() < 1e-10);
    assert_eq!(g.product_id(), 2);
}

#[test]
fn test_total_price() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "A".into(), cost: 10.0 }),
        Box::new(Gadget { id: 2, name: "B".into(), cost: 20.0 }),
    ];
    assert!((total_price(&products) - 30.0).abs() < 1e-10);
}

#[test]
fn test_total_price_empty() {
    let products: Vec<Box<dyn Product>> = vec![];
    assert_eq!(total_price(&products), 0.0);
}

#[test]
fn test_describe_all_products() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "Rust".into(), cost: 10.0 }),
        Box::new(Gadget { id: 2, name: "Phone".into(), cost: 20.0 }),
    ];
    let descs = describe_all(&products);
    assert_eq!(descs, vec!["Book: Rust", "Gadget: Phone"]);
}

#[test]
fn test_find_product_by_id_found() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "Rust".into(), cost: 10.0 }),
        Box::new(Gadget { id: 2, name: "Phone".into(), cost: 20.0 }),
    ];
    assert_eq!(find_product_by_id(&products, 2), Some("Gadget: Phone".to_string()));
}

#[test]
fn test_find_product_by_id_not_found() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "Rust".into(), cost: 10.0 }),
    ];
    assert_eq!(find_product_by_id(&products, 99), None);
}

#[test]
fn test_products_above_price() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "Cheap".into(), cost: 5.0 }),
        Box::new(Book { id: 2, title: "Expensive".into(), cost: 50.0 }),
        Box::new(Gadget { id: 3, name: "Mid".into(), cost: 15.0 }),
    ];
    let result = products_above_price(&products, 10.0);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"Book: Expensive".to_string()));
    assert!(result.contains(&"Gadget: Mid".to_string()));
}

#[test]
fn test_cheapest_product() {
    let products: Vec<Box<dyn Product>> = vec![
        Box::new(Book { id: 1, title: "A".into(), cost: 30.0 }),
        Box::new(Gadget { id: 2, name: "B".into(), cost: 5.0 }),
        Box::new(Book { id: 3, title: "C".into(), cost: 20.0 }),
    ];
    assert_eq!(cheapest_product(&products), Some("Gadget: B".to_string()));
}

#[test]
fn test_cheapest_product_empty() {
    let products: Vec<Box<dyn Product>> = vec![];
    assert_eq!(cheapest_product(&products), None);
}

// ============================================
// Topic 10: Closure Trait Objects
// ============================================

#[test]
fn test_apply_transform() {
    let double: TransformFn = Box::new(|x| x * 2);
    assert_eq!(apply_transform(5, &double), 10);
}

#[test]
fn test_apply_pipeline() {
    let transforms: Vec<TransformFn> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 3),
        Box::new(|x| x - 2),
    ];
    // (5 + 1) * 3 - 2 = 16
    assert_eq!(apply_pipeline(5, &transforms), 16);
}

#[test]
fn test_apply_pipeline_empty() {
    let transforms: Vec<TransformFn> = vec![];
    assert_eq!(apply_pipeline(42, &transforms), 42);
}

#[test]
fn test_transform_registry_register_and_apply() {
    let mut reg = TransformRegistry::new();
    reg.register("double", Box::new(|x| x * 2));
    reg.register("negate", Box::new(|x| -x));
    assert_eq!(reg.apply("double", 5), Some(10));
    assert_eq!(reg.apply("negate", 3), Some(-3));
    assert_eq!(reg.apply("unknown", 1), None);
}

#[test]
fn test_transform_registry_apply_all() {
    let mut reg = TransformRegistry::new();
    reg.register("double", Box::new(|x| x * 2));
    reg.register("square", Box::new(|x| x * x));
    let results = reg.apply_all(3);
    assert_eq!(results, vec![("double".to_string(), 6), ("square".to_string(), 9)]);
}

#[test]
fn test_transform_registry_count_and_names() {
    let mut reg = TransformRegistry::new();
    reg.register("a", Box::new(|x| x));
    reg.register("b", Box::new(|x| x));
    assert_eq!(reg.count(), 2);
    assert_eq!(reg.names(), vec!["a", "b"]);
}

#[test]
fn test_transform_registry_default() {
    let reg = TransformRegistry::default();
    assert_eq!(reg.count(), 0);
}

#[test]
fn test_filter_with_predicate() {
    let pred: PredicateFn = Box::new(|x| x > 3);
    let result = filter_with_predicate(&[1, 2, 3, 4, 5], &pred);
    assert_eq!(result, vec![4, 5]);
}

#[test]
fn test_and_predicate() {
    let gt2: PredicateFn = Box::new(|x| x > 2);
    let lt5: PredicateFn = Box::new(|x| x < 5);
    let combined = and_predicate(gt2, lt5);
    let result = filter_with_predicate(&[1, 2, 3, 4, 5, 6], &combined);
    assert_eq!(result, vec![3, 4]);
}

#[test]
fn test_or_predicate() {
    let eq1: PredicateFn = Box::new(|x| x == 1);
    let eq5: PredicateFn = Box::new(|x| x == 5);
    let combined = or_predicate(eq1, eq5);
    let result = filter_with_predicate(&[1, 2, 3, 4, 5], &combined);
    assert_eq!(result, vec![1, 5]);
}

#[test]
fn test_not_predicate() {
    let is_even: PredicateFn = Box::new(|x| x % 2 == 0);
    let is_odd = not_predicate(is_even);
    let result = filter_with_predicate(&[1, 2, 3, 4, 5], &is_odd);
    assert_eq!(result, vec![1, 3, 5]);
}

#[test]
fn test_filter_with_predicate_empty() {
    let pred: PredicateFn = Box::new(|_| true);
    let result = filter_with_predicate(&[], &pred);
    assert!(result.is_empty());
}
