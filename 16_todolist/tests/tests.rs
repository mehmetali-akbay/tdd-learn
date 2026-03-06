use todolist::*;

// ===== Topic 1: TodoItem =====

#[test]
fn test_new_item() {
    let item = TodoItem::new(1, "Buy groceries");
    assert_eq!(item.id, 1);
    assert_eq!(item.title, "Buy groceries");
    assert!(!item.completed);
    assert_eq!(item.priority, Priority::Medium);
}

#[test]
fn test_item_with_priority() {
    let item = TodoItem::new(1, "Urgent").with_priority(Priority::High);
    assert_eq!(item.priority, Priority::High);
}

#[test]
fn test_item_with_tags() {
    let item = TodoItem::new(1, "Fix bug").with_tags(&["work", "code"]);
    assert!(item.has_tag("work"));
    assert!(!item.has_tag("personal"));
}

#[test]
fn test_item_toggle() {
    let mut item = TodoItem::new(1, "Test");
    assert!(!item.completed);
    item.toggle();
    assert!(item.completed);
    item.toggle();
    assert!(!item.completed);
}

#[test]
fn test_item_display() {
    let item = TodoItem::new(1, "Buy milk");
    let s = format!("{}", item);
    assert!(s.contains("Buy milk"));
    assert!(s.contains("#1"));
}

#[test]
fn test_priority_display() {
    assert_eq!(format!("{}", Priority::High), "high");
    assert_eq!(format!("{}", Priority::Low), "low");
}

// ===== Topic 2: TodoList CRUD =====

#[test]
fn test_list_add() {
    let mut list = TodoList::new();
    let id1 = list.add("First");
    let id2 = list.add("Second");
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(list.len(), 2);
}

#[test]
fn test_list_add_with_priority() {
    let mut list = TodoList::new();
    let id = list.add_with_priority("Important", Priority::High);
    assert_eq!(list.get(id).unwrap().priority, Priority::High);
}

#[test]
fn test_list_get() {
    let mut list = TodoList::new();
    let id = list.add("Find me");
    assert_eq!(list.get(id).unwrap().title, "Find me");
    assert!(list.get(999).is_none());
}

#[test]
fn test_list_remove() {
    let mut list = TodoList::new();
    let id = list.add("Remove me");
    let removed = list.remove(id).unwrap();
    assert_eq!(removed.title, "Remove me");
    assert!(list.is_empty());
    assert!(list.remove(id).is_none());
}

#[test]
fn test_list_toggle() {
    let mut list = TodoList::new();
    let id = list.add("Toggle me");
    assert!(list.toggle(id));
    assert!(list.get(id).unwrap().completed);
    assert!(!list.toggle(999));
}

// ===== Topic 3: Filtering =====

#[test]
fn test_completed_pending() {
    let mut list = TodoList::new();
    list.add("A");
    let id = list.add("B");
    list.toggle(id);
    assert_eq!(list.completed().len(), 1);
    assert_eq!(list.pending().len(), 1);
}

#[test]
fn test_filter_by_priority() {
    let mut list = TodoList::new();
    list.add_with_priority("Low task", Priority::Low);
    list.add_with_priority("High task", Priority::High);
    list.add_with_priority("Another high", Priority::High);
    assert_eq!(list.by_priority(Priority::High).len(), 2);
    assert_eq!(list.by_priority(Priority::Low).len(), 1);
}

#[test]
fn test_filter_by_tag() {
    let mut list = TodoList::new();
    let id1 = list.add("Work task");
    let id2 = list.add("Personal task");
    list.get_mut(id1).unwrap().tags = vec!["work".to_string()];
    list.get_mut(id2).unwrap().tags = vec!["personal".to_string()];
    assert_eq!(list.by_tag("work").len(), 1);
    assert_eq!(list.by_tag("work")[0].title, "Work task");
}

#[test]
fn test_search() {
    let mut list = TodoList::new();
    list.add("Buy groceries");
    list.add("Buy milk");
    list.add("Fix car");
    assert_eq!(list.search("buy").len(), 2);
    assert_eq!(list.search("fix").len(), 1);
    assert_eq!(list.search("xyz").len(), 0);
}

#[test]
fn test_counts() {
    let mut list = TodoList::new();
    list.add("A");
    let id = list.add("B");
    list.toggle(id);
    assert_eq!(list.completed_count(), 1);
    assert_eq!(list.pending_count(), 1);
}

// ===== Topic 4: Sorting =====

#[test]
fn test_sorted_by_priority() {
    let mut list = TodoList::new();
    list.add_with_priority("Low", Priority::Low);
    list.add_with_priority("High", Priority::High);
    list.add_with_priority("Medium", Priority::Medium);
    let sorted = list.sorted_by_priority();
    assert_eq!(sorted[0].priority, Priority::High);
    assert_eq!(sorted[2].priority, Priority::Low);
}

#[test]
fn test_sorted_by_title() {
    let mut list = TodoList::new();
    list.add("Charlie");
    list.add("Alice");
    list.add("Bob");
    let sorted = list.sorted_by_title();
    assert_eq!(sorted[0].title, "Alice");
    assert_eq!(sorted[1].title, "Bob");
    assert_eq!(sorted[2].title, "Charlie");
}

#[test]
fn test_sorted_by_status_priority() {
    let mut list = TodoList::new();
    let id1 = list.add_with_priority("Done low", Priority::Low);
    list.add_with_priority("Pending high", Priority::High);
    list.add_with_priority("Pending low", Priority::Low);
    list.toggle(id1);
    let sorted = list.sorted_by_status_priority();
    // Pending first, high priority first among pending
    assert_eq!(sorted[0].title, "Pending high");
    assert_eq!(sorted[1].title, "Pending low");
    assert_eq!(sorted[2].title, "Done low"); // completed last
}

// ===== Topic 5: Serialization =====

#[test]
fn test_item_to_line() {
    let item = TodoItem::new(1, "Buy milk")
        .with_priority(Priority::High)
        .with_tags(&["shop"]);
    assert_eq!(item.to_line(), "1|Buy milk|false|high|shop");
}

#[test]
fn test_item_from_line() {
    let item = TodoItem::from_line("1|Buy milk|false|high|shop").unwrap();
    assert_eq!(item.id, 1);
    assert_eq!(item.title, "Buy milk");
    assert!(!item.completed);
    assert_eq!(item.priority, Priority::High);
    assert_eq!(item.tags, vec!["shop"]);
}

#[test]
fn test_item_from_line_invalid() {
    assert!(TodoItem::from_line("invalid").is_none());
    assert!(TodoItem::from_line("1|title|false|badpriority|").is_none());
}

#[test]
fn test_list_roundtrip() {
    let mut list = TodoList::new();
    list.add_with_priority("Task A", Priority::High);
    let id = list.add("Task B");
    list.toggle(id);
    let serialized = list.to_string_repr();
    let restored = TodoList::from_string_repr(&serialized);
    assert_eq!(restored.len(), 2);
    assert_eq!(restored.get(1).unwrap().title, "Task A");
    assert!(restored.get(2).unwrap().completed);
}

// ===== Topic 6: Stats & Bulk =====

#[test]
fn test_stats() {
    let mut list = TodoList::new();
    list.add_with_priority("A", Priority::High);
    list.add("B");
    let id = list.add("C");
    list.toggle(id);
    let stats = list.stats();
    assert_eq!(stats.total, 3);
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.pending, 2);
    assert_eq!(stats.high_priority, 1);
    assert!((stats.completion_rate - 1.0 / 3.0).abs() < 0.01);
}

#[test]
fn test_stats_empty() {
    let list = TodoList::new();
    let stats = list.stats();
    assert_eq!(stats.total, 0);
    assert_eq!(stats.completion_rate, 0.0);
}

#[test]
fn test_complete_all() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.complete_all();
    assert_eq!(list.completed_count(), 2);
}

#[test]
fn test_clear_completed() {
    let mut list = TodoList::new();
    list.add("Keep");
    let id = list.add("Remove");
    list.toggle(id);
    let removed = list.clear_completed();
    assert_eq!(removed, 1);
    assert_eq!(list.len(), 1);
    assert_eq!(list.all()[0].title, "Keep");
}

#[test]
fn test_set_priority_bulk() {
    let mut list = TodoList::new();
    let id1 = list.add("A");
    let id2 = list.add("B");
    list.add("C");
    let updated = list.set_priority_bulk(&[id1, id2], Priority::High);
    assert_eq!(updated, 2);
    assert_eq!(list.get(id1).unwrap().priority, Priority::High);
    assert_eq!(list.get(id2).unwrap().priority, Priority::High);
}

// ===== Edge Cases =====

#[test]
fn test_empty_list_operations() {
    let list = TodoList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.completed().len(), 0);
    assert_eq!(list.pending().len(), 0);
    assert_eq!(list.completed_count(), 0);
    assert_eq!(list.pending_count(), 0);
}

#[test]
fn test_remove_nonexistent() {
    let mut list = TodoList::new();
    assert!(list.remove(999).is_none());
}

#[test]
fn test_search_empty_query() {
    let mut list = TodoList::new();
    list.add("Something");
    assert_eq!(list.search("").len(), 1); // empty query matches all
}

#[test]
fn test_get_mut_modify() {
    let mut list = TodoList::new();
    let id = list.add("Original");
    list.get_mut(id).unwrap().title = "Modified".to_string();
    assert_eq!(list.get(id).unwrap().title, "Modified");
}

#[test]
fn test_complete_all_already_done() {
    let mut list = TodoList::new();
    let id = list.add("Done");
    list.toggle(id);
    list.complete_all();
    assert_eq!(list.completed_count(), 1);
}

#[test]
fn test_clear_completed_none() {
    let mut list = TodoList::new();
    list.add("Pending");
    let removed = list.clear_completed();
    assert_eq!(removed, 0);
    assert_eq!(list.len(), 1);
}
