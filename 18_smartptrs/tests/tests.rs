use smartptrs::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

// ===== Topic 1: Box<T> — Heap Allocation =====

#[test]
fn test_list_push_and_len() {
    let list = List::new().push(3).push(2).push(1);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
}

#[test]
fn test_list_head() {
    let list = List::new().push(3).push(2).push(1);
    assert_eq!(list.head(), Some(&1));
    assert_eq!(List::<i32>::new().head(), None);
}

#[test]
fn test_list_to_vec() {
    let list = List::new().push(3).push(2).push(1);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn test_list_empty() {
    let list = List::<i32>::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.to_vec(), Vec::<i32>::new());
}

#[test]
fn test_list_from_vec() {
    let list = List::from_vec(&[1, 2, 3]);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
    assert_eq!(list.head(), Some(&1));
    assert_eq!(list.len(), 3);
}

#[test]
fn test_list_reverse() {
    let list = List::from_vec(&[1, 2, 3]);
    let rev = list.reverse();
    assert_eq!(rev.to_vec(), vec![3, 2, 1]);
}

#[test]
fn test_list_append() {
    let a = List::from_vec(&[1, 2]);
    let b = List::from_vec(&[3, 4]);
    let combined = a.append(&b);
    assert_eq!(combined.to_vec(), vec![1, 2, 3, 4]);
}

#[test]
fn test_list_nth() {
    let list = List::from_vec(&[10, 20, 30]);
    assert_eq!(list.nth(0), Some(&10));
    assert_eq!(list.nth(2), Some(&30));
    assert_eq!(list.nth(5), None);
}

#[test]
fn test_list_map() {
    let list = List::from_vec(&[1, 2, 3]);
    let doubled = list.map(|x| x * 2);
    assert_eq!(doubled.to_vec(), vec![2, 4, 6]);
}

#[test]
fn test_list_contains() {
    let list = List::from_vec(&[1, 2, 3]);
    assert!(list.contains(&2));
    assert!(!list.contains(&5));
}

#[test]
fn test_tree_sum() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree.sum(), 6);
}

#[test]
fn test_tree_count_and_depth() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree.count_leaves(), 3);
    assert_eq!(tree.depth(), 2);
}

#[test]
fn test_tree_single_leaf() {
    let tree = Tree::Leaf(42);
    assert_eq!(tree.sum(), 42);
    assert_eq!(tree.count_leaves(), 1);
    assert_eq!(tree.depth(), 0);
}

#[test]
fn test_tree_flatten() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Leaf(3)),
    );
    assert_eq!(tree.flatten(), vec![&1, &2, &3]);
}

#[test]
fn test_tree_map_leaves() {
    let tree = Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)));
    let doubled = tree.map_leaves(&|x| x * 2);
    assert_eq!(doubled, Tree::Node(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(4))));
}

#[test]
fn test_tree_contains() {
    let tree = Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)));
    assert!(tree.contains(&1));
    assert!(!tree.contains(&5));
}

#[test]
fn test_tree_max_min() {
    let tree = Tree::Node(
        Box::new(Tree::Node(Box::new(Tree::Leaf(3)), Box::new(Tree::Leaf(7)))),
        Box::new(Tree::Leaf(1)),
    );
    assert_eq!(tree.max_leaf(), 7);
    assert_eq!(tree.min_leaf(), 1);
}

#[test]
fn test_trait_object_shapes() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    assert_eq!(shapes[0].name(), "circle");
    assert_eq!(shapes[1].name(), "square");
    assert!((total_area(&shapes) - (std::f64::consts::PI + 4.0)).abs() < 0.001);
}

#[test]
fn test_shape_areas() {
    let circle = Circle { radius: 2.0 };
    assert!((circle.area() - 4.0 * std::f64::consts::PI).abs() < 0.001);
    let square = Square { side: 5.0 };
    assert!((square.area() - 25.0).abs() < 0.001);
}

#[test]
fn test_total_area_empty() {
    let shapes: Vec<Box<dyn Shape>> = vec![];
    assert!((total_area(&shapes) - 0.0).abs() < 0.001);
}

#[test]
fn test_largest_shape_name() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 10.0 }),
    ];
    assert_eq!(largest_shape_name(&shapes), Some("square"));
    let empty: Vec<Box<dyn Shape>> = vec![];
    assert_eq!(largest_shape_name(&empty), None);
}

#[test]
fn test_describe_shape() {
    let c = Circle { radius: 2.0 };
    let desc = describe_shape(&c);
    assert!(desc.starts_with("circle (area="));
    assert!(desc.contains("12.57"));
}

// ===== Topic 2: Rc<T> — Shared Ownership =====

#[test]
fn test_shared_config() {
    let config = SharedConfig::new("app", true);
    assert_eq!(Rc::strong_count(&config), 1);
    let c1 = Component::new("ui", Rc::clone(&config));
    let c2 = Component::new("api", Rc::clone(&config));
    assert_eq!(Rc::strong_count(&config), 3);
    assert!(c1.is_debug());
    assert_eq!(c2.config.name, "app");
}

#[test]
fn test_config_summary() {
    let config = SharedConfig::new("myapp", true);
    assert_eq!(config.summary(), "Config 'myapp' (debug=on)");
    let config2 = SharedConfig::new("prod", false);
    assert_eq!(config2.summary(), "Config 'prod' (debug=off)");
}

#[test]
fn test_component_describe() {
    let config = SharedConfig::new("app", false);
    let comp = Component::new("ui", Rc::clone(&config));
    let desc = comp.describe();
    assert!(desc.contains("Component 'ui'"));
    assert!(desc.contains("Config 'app'"));
}

#[test]
fn test_rc_drops() {
    let config = SharedConfig::new("test", false);
    {
        let _c = Component::new("temp", Rc::clone(&config));
        assert_eq!(Rc::strong_count(&config), 2);
    }
    assert_eq!(Rc::strong_count(&config), 1);
}

#[test]
fn test_graph_node() {
    let leaf1 = GraphNode::leaf(1);
    let leaf2 = GraphNode::leaf(2);
    let shared = GraphNode::leaf(3);
    let parent1 = GraphNode::with_children(10, vec![Rc::clone(&leaf1), Rc::clone(&shared)]);
    let parent2 = GraphNode::with_children(20, vec![Rc::clone(&leaf2), Rc::clone(&shared)]);
    let root = GraphNode::with_children(0, vec![parent1, parent2]);
    assert_eq!(GraphNode::count_nodes(&root), 7);
    assert_eq!(Rc::strong_count(&shared), 3);
}

#[test]
fn test_graph_leaf_count() {
    let leaf = GraphNode::leaf(42);
    assert_eq!(GraphNode::count_nodes(&leaf), 1);
}

#[test]
fn test_graph_max_depth() {
    let leaf = GraphNode::leaf(1);
    assert_eq!(GraphNode::max_depth(&leaf), 0);
    let deep = GraphNode::with_children(
        0,
        vec![GraphNode::with_children(1, vec![GraphNode::leaf(2)])],
    );
    assert_eq!(GraphNode::max_depth(&deep), 2);
}

#[test]
fn test_graph_all_values() {
    let root = GraphNode::with_children(
        1,
        vec![GraphNode::leaf(2), GraphNode::leaf(3)],
    );
    let values = GraphNode::all_values(&root);
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_graph_find_value() {
    let root = GraphNode::with_children(
        1,
        vec![GraphNode::leaf(2), GraphNode::leaf(3)],
    );
    assert!(GraphNode::find_value(&root, 3));
    assert!(!GraphNode::find_value(&root, 99));
}

// ===== Topic 3: RefCell<T> — Interior Mutability =====

#[test]
fn test_logger() {
    let logger = Logger::new();
    logger.log("hello");
    logger.log("world");
    assert_eq!(logger.count(), 2);
    assert_eq!(logger.messages(), vec!["hello", "world"]);
}

#[test]
fn test_logger_clear() {
    let logger = Logger::new();
    logger.log("test");
    logger.clear();
    assert_eq!(logger.count(), 0);
}

#[test]
fn test_logger_empty() {
    let logger = Logger::new();
    assert_eq!(logger.count(), 0);
    assert_eq!(logger.messages(), Vec::<String>::new());
}

#[test]
fn test_logger_last() {
    let logger = Logger::new();
    assert_eq!(logger.last(), None);
    logger.log("first");
    logger.log("second");
    assert_eq!(logger.last(), Some("second".to_string()));
}

#[test]
fn test_logger_contains_message() {
    let logger = Logger::new();
    logger.log("error: file not found");
    logger.log("info: started");
    assert!(logger.contains_message("error"));
    assert!(!logger.contains_message("warning"));
}

#[test]
fn test_logger_filter_messages() {
    let logger = Logger::new();
    logger.log("error: bad");
    logger.log("info: ok");
    logger.log("error: worse");
    let errors = logger.filter_messages(|m| m.starts_with("error"));
    assert_eq!(errors.len(), 2);
    assert!(errors[0].contains("bad"));
}

#[test]
fn test_counter() {
    let counter = Counter::new(0);
    counter.borrow_mut().increment();
    counter.borrow_mut().increment();
    counter.borrow_mut().decrement();
    assert_eq!(counter.borrow().value(), 1);
}

#[test]
fn test_shared_counter() {
    let counter = Counter::new(10);
    let c2 = Rc::clone(&counter);
    c2.borrow_mut().increment();
    assert_eq!(counter.borrow().value(), 11);
}

#[test]
fn test_counter_reset() {
    let counter = Counter::new(5);
    counter.borrow_mut().increment();
    counter.borrow_mut().reset(0);
    assert_eq!(counter.borrow().value(), 0);
}

#[test]
fn test_get_shared_value() {
    let counter = Counter::new(42);
    assert_eq!(get_shared_value(&counter), 42);
    counter.borrow_mut().increment();
    assert_eq!(get_shared_value(&counter), 43);
}

// ===== Topic 4: Rc<RefCell<T>> — Shared Mutable State =====

#[test]
fn test_bank_deposit() {
    let acc = BankAccount::new("Alice", 100.0);
    assert_eq!(acc.deposit(50.0), Ok(150.0));
    assert!((acc.get_balance() - 150.0).abs() < 0.01);
}

#[test]
fn test_bank_withdraw() {
    let acc = BankAccount::new("Bob", 100.0);
    assert_eq!(acc.withdraw(30.0), Ok(70.0));
    assert!(acc.withdraw(200.0).is_err());
    assert!(acc.withdraw(-10.0).is_err());
}

#[test]
fn test_bank_deposit_negative() {
    let acc = BankAccount::new("Test", 100.0);
    assert!(acc.deposit(-10.0).is_err());
}

#[test]
fn test_bank_zero_balance_withdraw() {
    let acc = BankAccount::new("Test", 0.0);
    assert!(acc.withdraw(1.0).is_err());
    assert!((acc.get_balance() - 0.0).abs() < 0.01);
}

#[test]
fn test_bank_transfer() {
    let alice = BankAccount::new("Alice", 100.0);
    let bob = BankAccount::new("Bob", 50.0);
    assert!(transfer(&alice, &bob, 30.0).is_ok());
    assert!((alice.get_balance() - 70.0).abs() < 0.01);
    assert!((bob.get_balance() - 80.0).abs() < 0.01);
}

#[test]
fn test_bank_transfer_insufficient() {
    let alice = BankAccount::new("Alice", 10.0);
    let bob = BankAccount::new("Bob", 50.0);
    assert!(transfer(&alice, &bob, 100.0).is_err());
    assert!((alice.get_balance() - 10.0).abs() < 0.01);
    assert!((bob.get_balance() - 50.0).abs() < 0.01);
}

#[test]
fn test_shared_account() {
    let acc = BankAccount::new("Shared", 100.0);
    let acc2 = Rc::clone(&acc);
    acc.deposit(50.0).unwrap();
    acc2.withdraw(20.0).unwrap();
    assert!((acc.get_balance() - 130.0).abs() < 0.01);
}

#[test]
fn test_balance_summary() {
    let acc = BankAccount::new("Alice", 123.45);
    assert_eq!(acc.balance_summary(), "Alice: $123.45");
}

#[test]
fn test_total_balance() {
    let a = BankAccount::new("A", 100.0);
    let b = BankAccount::new("B", 200.0);
    let c = BankAccount::new("C", 50.0);
    assert!((total_balance(&[a, b, c]) - 350.0).abs() < 0.01);
}

#[test]
fn test_shared_cache() {
    let cache = SharedCache::new();
    assert!(cache.is_empty());
    cache.insert("key1", "val1");
    cache.insert("key2", "val2");
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get("key1"), Some("val1".to_string()));
    assert!(cache.contains("key2"));
    assert!(!cache.contains("key3"));
}

#[test]
fn test_shared_cache_remove_clear() {
    let cache = SharedCache::new();
    cache.insert("a", "1");
    cache.insert("b", "2");
    assert_eq!(cache.remove("a"), Some("1".to_string()));
    assert_eq!(cache.len(), 1);
    cache.clear();
    assert!(cache.is_empty());
}

#[test]
fn test_shared_cache_shared() {
    let cache = SharedCache::new();
    let cache2 = Rc::clone(&cache);
    cache.insert("x", "10");
    assert_eq!(cache2.get("x"), Some("10".to_string()));
}

// ===== Topic 5: Drop Trait & Custom Smart Pointers =====

#[test]
fn test_tracked_drop() {
    let counter = Rc::new(Cell::new(0usize));
    {
        let _a = Tracked::new(1, Rc::clone(&counter));
        let _b = Tracked::new(2, Rc::clone(&counter));
        assert_eq!(counter.get(), 2);
    }
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_tracked_instance_count() {
    let counter = Rc::new(Cell::new(0usize));
    let a = Tracked::new("x", Rc::clone(&counter));
    assert_eq!(a.instance_count(), 1);
    let b = Tracked::new("y", Rc::clone(&counter));
    assert_eq!(b.instance_count(), 2);
    drop(a);
    assert_eq!(b.instance_count(), 1);
}

#[test]
fn test_history() {
    let mut h = History::new(1, 3);
    h.set(2);
    h.set(3);
    assert_eq!(*h.get(), 3);
    assert_eq!(h.history_len(), 2);
}

#[test]
fn test_history_undo() {
    let mut h = History::new("a", 3);
    h.set("b");
    h.set("c");
    assert!(h.undo());
    assert_eq!(*h.get(), "b");
    assert!(h.undo());
    assert_eq!(*h.get(), "a");
    assert!(!h.undo());
}

#[test]
fn test_history_max() {
    let mut h = History::new(0, 2);
    h.set(1);
    h.set(2);
    h.set(3);
    assert_eq!(h.history_len(), 2);
    assert_eq!(*h.get(), 3);
}

#[test]
fn test_history_no_undo_on_fresh() {
    let h = History::new(42, 5);
    assert_eq!(*h.get(), 42);
    assert_eq!(h.history_len(), 0);
}

#[test]
fn test_history_can_undo() {
    let mut h = History::new(1, 5);
    assert!(!h.can_undo());
    h.set(2);
    assert!(h.can_undo());
}

#[test]
fn test_history_peek_prev() {
    let mut h = History::new(1, 5);
    assert_eq!(h.peek_prev(), None);
    h.set(2);
    assert_eq!(h.peek_prev(), Some(&1));
    h.set(3);
    assert_eq!(h.peek_prev(), Some(&2));
}

#[test]
fn test_history_get_history() {
    let mut h = History::new(1, 5);
    h.set(2);
    h.set(3);
    assert_eq!(h.get_history(), &[1, 2]);
}

#[test]
fn test_verbose_create_drop() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _v = Verbose::new(42, Rc::clone(&log));
        assert_eq!(log.borrow().len(), 1);
        assert!(log.borrow()[0].contains("Created"));
    }
    assert_eq!(log.borrow().len(), 2);
    assert!(log.borrow()[1].contains("Dropped"));
}

// ===== Topic 6: Observer Pattern & Callbacks =====

#[test]
fn test_event_emitter() {
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let log_clone = Rc::clone(&log);
    let mut emitter = EventEmitter::new();
    emitter.on(Rc::new(RefCell::new(move |event: &str| {
        log_clone.borrow_mut().push(event.to_string());
    })));
    emitter.emit("click");
    emitter.emit("hover");
    assert_eq!(log.borrow().len(), 2);
    assert_eq!(log.borrow()[0], "click");
}

#[test]
fn test_event_emitter_multiple() {
    let count = Rc::new(Cell::new(0u32));
    let c1 = Rc::clone(&count);
    let c2 = Rc::clone(&count);
    let mut emitter = EventEmitter::new();
    emitter.on(Rc::new(RefCell::new(move |_: &str| {
        c1.set(c1.get() + 1);
    })));
    emitter.on(Rc::new(RefCell::new(move |_: &str| {
        c2.set(c2.get() + 10);
    })));
    assert_eq!(emitter.listener_count(), 2);
    emitter.emit("test");
    assert_eq!(count.get(), 11);
}

#[test]
fn test_event_emitter_no_listeners() {
    let emitter = EventEmitter::new();
    emitter.emit("test");
    assert_eq!(emitter.listener_count(), 0);
    assert!(!emitter.has_listeners());
}

#[test]
fn test_event_emitter_clear_listeners() {
    let mut emitter = EventEmitter::new();
    let cb: Rc<RefCell<dyn FnMut(&str)>> = Rc::new(RefCell::new(|_: &str| {}));
    emitter.on(cb);
    assert!(emitter.has_listeners());
    emitter.clear_listeners();
    assert!(!emitter.has_listeners());
    assert_eq!(emitter.listener_count(), 0);
}

#[test]
fn test_event_bus_subscribe_publish() {
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let log_clone = Rc::clone(&log);
    let mut bus = EventBus::new();
    bus.subscribe(
        "click",
        Rc::new(RefCell::new(move |data: &str| {
            log_clone.borrow_mut().push(data.to_string());
        })),
    );
    bus.publish("click", "button1");
    bus.publish("hover", "nothing"); // no subscriber
    assert_eq!(log.borrow().len(), 1);
    assert_eq!(log.borrow()[0], "button1");
}

#[test]
fn test_event_bus_handler_count() {
    let mut bus = EventBus::new();
    let cb: Rc<RefCell<dyn FnMut(&str)>> = Rc::new(RefCell::new(|_: &str| {}));
    bus.subscribe("a", Rc::clone(&cb));
    bus.subscribe("a", Rc::clone(&cb));
    bus.subscribe("b", cb);
    assert_eq!(bus.handler_count("a"), 2);
    assert_eq!(bus.handler_count("b"), 1);
    assert_eq!(bus.handler_count("c"), 0);
}

#[test]
fn test_event_bus_event_names() {
    let mut bus = EventBus::new();
    let cb: Rc<RefCell<dyn FnMut(&str)>> = Rc::new(RefCell::new(|_: &str| {}));
    bus.subscribe("click", Rc::clone(&cb));
    bus.subscribe("hover", cb);
    assert_eq!(bus.event_names(), vec!["click", "hover"]);
}

// ===== Topic 7: Arc<T> & Mutex<T> — Thread-Safe Smart Pointers =====

#[test]
fn test_atomic_counter_basic() {
    let counter = AtomicCounter::new(0);
    counter.increment();
    counter.increment();
    counter.decrement();
    assert_eq!(counter.get(), 1);
}

#[test]
fn test_atomic_counter_add_reset() {
    let counter = AtomicCounter::new(10);
    counter.add(5);
    assert_eq!(counter.get(), 15);
    counter.reset(0);
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_atomic_counter_threaded() {
    let counter = AtomicCounter::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(counter.get(), 1000);
}

#[test]
fn test_shared_log_basic() {
    let log = SharedLog::new();
    assert!(log.is_empty());
    log.append("entry1");
    log.append("entry2");
    assert_eq!(log.len(), 2);
    assert_eq!(log.entries(), vec!["entry1", "entry2"]);
}

#[test]
fn test_shared_log_clear() {
    let log = SharedLog::new();
    log.append("x");
    log.clear();
    assert!(log.is_empty());
    assert_eq!(log.len(), 0);
}

#[test]
fn test_shared_log_threaded() {
    let log = SharedLog::new();
    let mut handles = vec![];
    for i in 0..5 {
        let l = Arc::clone(&log);
        handles.push(std::thread::spawn(move || {
            l.append(&format!("thread-{i}"));
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(log.len(), 5);
}

#[test]
fn test_concurrent_map_basic() {
    let map = ConcurrentMap::<i32>::new();
    assert!(map.is_empty());
    map.insert("a", 1);
    map.insert("b", 2);
    assert_eq!(map.get("a"), Some(1));
    assert_eq!(map.get("c"), None);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_concurrent_map_remove_keys() {
    let map = ConcurrentMap::<String>::new();
    map.insert("x", "hello".to_string());
    map.insert("y", "world".to_string());
    assert_eq!(map.keys(), vec!["x", "y"]);
    assert_eq!(map.remove("x"), Some("hello".to_string()));
    assert_eq!(map.len(), 1);
    assert!(!map.is_empty());
}

#[test]
fn test_concurrent_map_threaded() {
    let map = ConcurrentMap::<i32>::new();
    let mut handles = vec![];
    for i in 0..5 {
        let m = Arc::clone(&map);
        handles.push(std::thread::spawn(move || {
            m.insert(&format!("key-{i}"), i);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(map.len(), 5);
}
