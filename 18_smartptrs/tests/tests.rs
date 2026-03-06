use smartptrs::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// ===== Topic 1: Box<T> =====

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
fn test_list_empty() {
    let list = List::<i32>::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.to_vec(), Vec::<i32>::new());
}

#[test]
fn test_list_sum_via_vec() {
    let list = List::new().push(10).push(20).push(30);
    let sum: i32 = list.to_vec().iter().sum();
    assert_eq!(sum, 60);
    assert_eq!(List::<i32>::new().to_vec().len(), 0);
}

#[test]
fn test_tree_single_leaf() {
    let tree = Tree::Leaf(42);
    assert_eq!(tree.sum(), 42);
    assert_eq!(tree.count_leaves(), 1);
    assert_eq!(tree.depth(), 0);
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

// ===== Topic 2: Rc<T> =====

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
    // shared is used twice but counted in each subtree
    assert_eq!(GraphNode::count_nodes(&root), 7);
    assert_eq!(Rc::strong_count(&shared), 3); // root children + 2 parents
}

// ===== Topic 3: RefCell<T> =====

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

// ===== Topic 4: Rc<RefCell<T>> — BankAccount =====

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
    assert!(acc.withdraw(200.0).is_err()); // insufficient
    assert!(acc.withdraw(-10.0).is_err()); // negative
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
    // Balances unchanged on failed transfer
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

// ===== Topic 5: Drop & History =====

#[test]
fn test_tracked_drop() {
    let counter = Rc::new(Cell::new(0usize));
    {
        let _a = Tracked::new(1, Rc::clone(&counter));
        let _b = Tracked::new(2, Rc::clone(&counter));
        assert_eq!(counter.get(), 2);
    }
    assert_eq!(counter.get(), 0); // both dropped
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
    assert!(!h.undo()); // nothing left
}

#[test]
fn test_history_max() {
    let mut h = History::new(0, 2);
    h.set(1);
    h.set(2);
    h.set(3); // history should be [2, 3 is current], oldest (0→1) dropped
    assert_eq!(h.history_len(), 2);
    assert_eq!(*h.get(), 3);
}

// ===== Topic 6: Observer =====

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
    assert_eq!(count.get(), 11); // 1 + 10
}

// ===== Edge Cases =====

#[test]
fn test_graph_leaf_count() {
    let leaf = GraphNode::leaf(42);
    assert_eq!(GraphNode::count_nodes(&leaf), 1);
}

#[test]
fn test_logger_empty() {
    let logger = Logger::new();
    assert_eq!(logger.count(), 0);
    assert_eq!(logger.messages(), Vec::<String>::new());
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
fn test_history_no_undo_on_fresh() {
    let h = History::new(42, 5);
    assert_eq!(*h.get(), 42);
    assert_eq!(h.history_len(), 0);
}

#[test]
fn test_event_emitter_no_listeners() {
    let emitter = EventEmitter::new();
    emitter.emit("test"); // should not panic
    assert_eq!(emitter.listener_count(), 0);
}
