use todolist::*;

// =====================================================================
// Topic 1: TodoItem — Struct Design
// =====================================================================

#[test]
fn test_new_item() {
    let item = TodoItem::new(1, "Buy groceries");
    assert_eq!(item.id, 1);
    assert_eq!(item.title, "Buy groceries");
    assert!(!item.completed);
    assert_eq!(item.priority, Priority::Medium);
    assert!(item.tags.is_empty());
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
    assert!(item.has_tag("code"));
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
    assert!(s.contains("☐")); // pending
}

#[test]
fn test_item_display_completed() {
    let mut item = TodoItem::new(2, "Done task");
    item.toggle();
    let s = format!("{}", item);
    assert!(s.contains("✓"));
}

#[test]
fn test_priority_display() {
    assert_eq!(format!("{}", Priority::High), "high");
    assert_eq!(format!("{}", Priority::Medium), "medium");
    assert_eq!(format!("{}", Priority::Low), "low");
}

#[test]
fn test_priority_ordering() {
    assert!(Priority::High > Priority::Medium);
    assert!(Priority::Medium > Priority::Low);
    assert!(Priority::Low < Priority::High);
}

#[test]
fn test_priority_from_str_opt() {
    assert_eq!(Priority::from_str_opt("high"), Some(Priority::High));
    assert_eq!(Priority::from_str_opt("medium"), Some(Priority::Medium));
    assert_eq!(Priority::from_str_opt("low"), Some(Priority::Low));
    assert_eq!(Priority::from_str_opt("invalid"), None);
}

#[test]
fn test_add_tag() {
    let mut item = TodoItem::new(1, "Task");
    item.add_tag("work");
    assert!(item.has_tag("work"));
    assert_eq!(item.tag_count(), 1);
}

#[test]
fn test_add_tag_no_duplicate() {
    let mut item = TodoItem::new(1, "Task");
    item.add_tag("work");
    item.add_tag("work");
    assert_eq!(item.tag_count(), 1);
}

#[test]
fn test_remove_tag() {
    let mut item = TodoItem::new(1, "Task").with_tags(&["a", "b", "c"]);
    assert!(item.remove_tag("b"));
    assert!(!item.has_tag("b"));
    assert_eq!(item.tag_count(), 2);
}

#[test]
fn test_remove_tag_missing() {
    let mut item = TodoItem::new(1, "Task");
    assert!(!item.remove_tag("nonexistent"));
}

#[test]
fn test_rename() {
    let mut item = TodoItem::new(1, "Old name");
    item.rename("New name");
    assert_eq!(item.title, "New name");
}

#[test]
fn test_clear_tags() {
    let mut item = TodoItem::new(1, "Task").with_tags(&["a", "b"]);
    item.clear_tags();
    assert_eq!(item.tag_count(), 0);
    assert!(item.tags.is_empty());
}

#[test]
fn test_is_high_priority() {
    let high = TodoItem::new(1, "H").with_priority(Priority::High);
    let low = TodoItem::new(2, "L").with_priority(Priority::Low);
    assert!(high.is_high_priority());
    assert!(!low.is_high_priority());
}

// =====================================================================
// Topic 2: TodoList — Collection Management
// =====================================================================

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
fn test_list_add_with_tags() {
    let mut list = TodoList::new();
    let id = list.add_with_tags("Tagged", &["work", "urgent"]);
    let item = list.get(id).unwrap();
    assert!(item.has_tag("work"));
    assert!(item.has_tag("urgent"));
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

#[test]
fn test_update_title() {
    let mut list = TodoList::new();
    let id = list.add("Original");
    assert!(list.update_title(id, "Updated"));
    assert_eq!(list.get(id).unwrap().title, "Updated");
}

#[test]
fn test_update_title_nonexistent() {
    let mut list = TodoList::new();
    assert!(!list.update_title(999, "Nope"));
}

#[test]
fn test_contains() {
    let mut list = TodoList::new();
    let id = list.add("Exists");
    assert!(list.contains(id));
    assert!(!list.contains(999));
}

#[test]
fn test_ids() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.add("C");
    assert_eq!(list.ids(), vec![1, 2, 3]);
}

#[test]
fn test_titles() {
    let mut list = TodoList::new();
    list.add("Alpha");
    list.add("Beta");
    assert_eq!(list.titles(), vec!["Alpha", "Beta"]);
}

#[test]
fn test_default() {
    let list = TodoList::default();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_list_ids_after_remove() {
    let mut list = TodoList::new();
    list.add("A"); // 1
    let id2 = list.add("B"); // 2
    list.add("C"); // 3
    list.remove(id2);
    assert_eq!(list.ids(), vec![1, 3]);
}

// =====================================================================
// Topic 3: Filtering & Searching
// =====================================================================

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
fn test_by_any_tag() {
    let mut list = TodoList::new();
    list.add_with_tags("Work", &["work"]);
    list.add_with_tags("Home", &["home"]);
    list.add_with_tags("Both", &["work", "home"]);
    list.add("Untagged");
    assert_eq!(list.by_any_tag(&["work"]).len(), 2);
    assert_eq!(list.by_any_tag(&["work", "home"]).len(), 3);
    assert_eq!(list.by_any_tag(&["missing"]).len(), 0);
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
fn test_search_case_insensitive() {
    let mut list = TodoList::new();
    list.add("Buy Groceries");
    assert_eq!(list.search("BUY").len(), 1);
    assert_eq!(list.search("groceries").len(), 1);
}

#[test]
fn test_filter_by_custom() {
    let mut list = TodoList::new();
    list.add_with_priority("High task", Priority::High);
    list.add("Medium task");
    list.add_with_priority("Low task", Priority::Low);
    let high_pending = list.filter_by(|item| item.is_high_priority() && !item.completed);
    assert_eq!(high_pending.len(), 1);
    assert_eq!(high_pending[0].title, "High task");
}

#[test]
fn test_find_first() {
    let mut list = TodoList::new();
    list.add("Alpha");
    list.add("Beta");
    list.add("Gamma");
    let found = list.find_first(|item| item.title.starts_with('B'));
    assert_eq!(found.unwrap().title, "Beta");
}

#[test]
fn test_find_first_none() {
    let mut list = TodoList::new();
    list.add("Alpha");
    assert!(list.find_first(|item| item.title == "Missing").is_none());
}

#[test]
fn test_pending_by_priority() {
    let mut list = TodoList::new();
    let id1 = list.add_with_priority("H1", Priority::High);
    list.add_with_priority("H2", Priority::High);
    list.add_with_priority("L1", Priority::Low);
    list.toggle(id1); // complete H1
    let pending_high = list.pending_by_priority(Priority::High);
    assert_eq!(pending_high.len(), 1);
    assert_eq!(pending_high[0].title, "H2");
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

// =====================================================================
// Topic 4: Sorting & Ordering
// =====================================================================

#[test]
fn test_sorted_by_priority() {
    let mut list = TodoList::new();
    list.add_with_priority("Low", Priority::Low);
    list.add_with_priority("High", Priority::High);
    list.add_with_priority("Medium", Priority::Medium);
    let sorted = list.sorted_by_priority();
    assert_eq!(sorted[0].priority, Priority::High);
    assert_eq!(sorted[1].priority, Priority::Medium);
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
    assert_eq!(sorted[0].title, "Pending high");
    assert_eq!(sorted[1].title, "Pending low");
    assert_eq!(sorted[2].title, "Done low");
}

#[test]
fn test_sorted_by_id() {
    let mut list = TodoList::new();
    list.add("C");
    list.add("A");
    list.add("B");
    let sorted = list.sorted_by_id();
    assert_eq!(sorted[0].id, 1);
    assert_eq!(sorted[1].id, 2);
    assert_eq!(sorted[2].id, 3);
}

#[test]
fn test_sorted_by_tag_count() {
    let mut list = TodoList::new();
    list.add_with_tags("Three tags", &["a", "b", "c"]);
    list.add("No tags");
    list.add_with_tags("One tag", &["x"]);
    let sorted = list.sorted_by_tag_count();
    assert_eq!(sorted[0].title, "Three tags");
    assert_eq!(sorted[1].title, "One tag");
    assert_eq!(sorted[2].title, "No tags");
}

#[test]
fn test_sorted_by_custom() {
    let mut list = TodoList::new();
    list.add("Zebra");
    list.add("Apple");
    list.add("Mango");
    // Sort by title length, then alphabetically
    let sorted = list.sorted_by_custom(|a, b| {
        a.title.len().cmp(&b.title.len()).then(a.title.cmp(&b.title))
    });
    assert_eq!(sorted[0].title, "Apple");
    assert_eq!(sorted[1].title, "Mango");
    assert_eq!(sorted[2].title, "Zebra");
}

#[test]
fn test_sort_empty_list() {
    let list = TodoList::new();
    assert!(list.sorted_by_priority().is_empty());
    assert!(list.sorted_by_title().is_empty());
    assert!(list.sorted_by_id().is_empty());
}

// =====================================================================
// Topic 5: Serialization — to/from string
// =====================================================================

#[test]
fn test_item_to_line() {
    let item = TodoItem::new(1, "Buy milk")
        .with_priority(Priority::High)
        .with_tags(&["shop"]);
    assert_eq!(item.to_line(), "1|Buy milk|false|high|shop");
}

#[test]
fn test_item_to_line_multiple_tags() {
    let item = TodoItem::new(3, "Task").with_tags(&["a", "b", "c"]);
    assert_eq!(item.to_line(), "3|Task|false|medium|a,b,c");
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
fn test_item_from_line_completed() {
    let item = TodoItem::from_line("5|Done|true|low|").unwrap();
    assert!(item.completed);
    assert!(item.tags.is_empty());
}

#[test]
fn test_item_from_line_invalid() {
    assert!(TodoItem::from_line("invalid").is_none());
    assert!(TodoItem::from_line("1|title|false|badpriority|").is_none());
    assert!(TodoItem::from_line("notanumber|title|false|high|").is_none());
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

#[test]
fn test_roundtrip_with_tags() {
    let mut list = TodoList::new();
    list.add_with_tags("Tagged task", &["work", "urgent"]);
    let serialized = list.to_string_repr();
    let restored = TodoList::from_string_repr(&serialized);
    let item = restored.get(1).unwrap();
    assert!(item.has_tag("work"));
    assert!(item.has_tag("urgent"));
}

#[test]
fn test_to_summary() {
    let mut list = TodoList::new();
    list.add("Task A");
    let id = list.add("Task B");
    list.toggle(id);
    let summary = list.to_summary();
    assert!(summary.contains("2 items"));
    assert!(summary.contains("1 completed"));
    assert!(summary.contains("1 pending"));
    assert!(summary.contains("50%"));
    assert!(summary.contains("Task A"));
    assert!(summary.contains("Task B"));
}

#[test]
fn test_to_summary_empty() {
    let list = TodoList::new();
    let summary = list.to_summary();
    assert!(summary.contains("0 items"));
    assert!(!summary.contains('%'));
}

// =====================================================================
// Topic 6: Statistics & Bulk Operations
// =====================================================================

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
    assert_eq!(list.pending_count(), 0);
}

#[test]
fn test_uncomplete_all() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.complete_all();
    assert_eq!(list.completed_count(), 2);
    list.uncomplete_all();
    assert_eq!(list.completed_count(), 0);
    assert_eq!(list.pending_count(), 2);
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

#[test]
fn test_add_tag_bulk() {
    let mut list = TodoList::new();
    let id1 = list.add("A");
    let id2 = list.add("B");
    list.add("C");
    let updated = list.add_tag_bulk(&[id1, id2], "urgent");
    assert_eq!(updated, 2);
    assert!(list.get(id1).unwrap().has_tag("urgent"));
    assert!(list.get(id2).unwrap().has_tag("urgent"));
    assert!(!list.get(3).unwrap().has_tag("urgent"));
}

#[test]
fn test_add_tag_bulk_nonexistent() {
    let mut list = TodoList::new();
    list.add("A");
    let updated = list.add_tag_bulk(&[99, 100], "tag");
    assert_eq!(updated, 0);
}

#[test]
fn test_remove_by_priority() {
    let mut list = TodoList::new();
    list.add_with_priority("Low1", Priority::Low);
    list.add_with_priority("High1", Priority::High);
    list.add_with_priority("Low2", Priority::Low);
    let removed = list.remove_by_priority(Priority::Low);
    assert_eq!(removed, 2);
    assert_eq!(list.len(), 1);
    assert_eq!(list.all()[0].title, "High1");
}

#[test]
fn test_remove_by_priority_none() {
    let mut list = TodoList::new();
    list.add("Medium");
    let removed = list.remove_by_priority(Priority::High);
    assert_eq!(removed, 0);
    assert_eq!(list.len(), 1);
}

#[test]
fn test_priority_distribution() {
    let mut list = TodoList::new();
    list.add_with_priority("H1", Priority::High);
    list.add_with_priority("H2", Priority::High);
    list.add("M1"); // default Medium
    list.add_with_priority("L1", Priority::Low);
    let dist = list.priority_distribution();
    assert_eq!(dist[&Priority::High], 2);
    assert_eq!(dist[&Priority::Medium], 1);
    assert_eq!(dist[&Priority::Low], 1);
}

// =====================================================================
// Topic 7: Iteration & Transformation
// =====================================================================

#[test]
fn test_iter() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    let titles: Vec<&str> = list.iter().map(|item| item.title.as_str()).collect();
    assert_eq!(titles, vec!["A", "B"]);
}

#[test]
fn test_iter_count() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.add("C");
    assert_eq!(list.iter().count(), 3);
}

#[test]
fn test_into_items() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    let items = list.into_items();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].title, "A");
    assert_eq!(items[1].title, "B");
}

#[test]
fn test_merge() {
    let mut list1 = TodoList::new();
    list1.add("From list1");
    let mut list2 = TodoList::new();
    list2.add("From list2");
    list2.add("Also list2");
    list1.merge(list2);
    assert_eq!(list1.len(), 3);
    assert_eq!(list1.titles(), vec!["From list1", "From list2", "Also list2"]);
}

#[test]
fn test_merge_ids_reassigned() {
    let mut list1 = TodoList::new();
    list1.add("A"); // id 1
    list1.add("B"); // id 2
    let mut list2 = TodoList::new();
    list2.add("C"); // was id 1 in list2
    list1.merge(list2);
    // After merge, C should get id 3 (not 1)
    assert_eq!(list1.ids(), vec![1, 2, 3]);
}

#[test]
fn test_map_titles() {
    let mut list = TodoList::new();
    list.add("hello world");
    list.add("foo bar");
    let upper = list.map_titles(|t| t.to_uppercase());
    assert_eq!(upper, vec!["HELLO WORLD", "FOO BAR"]);
}

#[test]
fn test_map_titles_with_length() {
    let mut list = TodoList::new();
    list.add("abc");
    list.add("defgh");
    let lengths = list.map_titles(|t| format!("{}({})", t, t.len()));
    assert_eq!(lengths, vec!["abc(3)", "defgh(5)"]);
}

#[test]
fn test_all_tags() {
    let mut list = TodoList::new();
    list.add_with_tags("A", &["work", "code"]);
    list.add_with_tags("B", &["home", "work"]);
    list.add("C");
    let tags = list.all_tags();
    assert_eq!(tags, vec!["code", "home", "work"]); // sorted, unique
}

#[test]
fn test_all_tags_empty() {
    let list = TodoList::new();
    assert!(list.all_tags().is_empty());
}

#[test]
fn test_most_common_tag() {
    let mut list = TodoList::new();
    list.add_with_tags("A", &["work", "code"]);
    list.add_with_tags("B", &["work", "home"]);
    list.add_with_tags("C", &["work"]);
    assert_eq!(list.most_common_tag(), Some("work".to_string()));
}

#[test]
fn test_most_common_tag_empty() {
    let list = TodoList::new();
    assert_eq!(list.most_common_tag(), None);
}

#[test]
fn test_apply_to_all() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.apply_to_all(|item| item.priority = Priority::High);
    assert!(list.all().iter().all(|item| item.priority == Priority::High));
}

#[test]
fn test_apply_to_all_add_tag() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.apply_to_all(|item| item.add_tag("bulk-tagged"));
    assert!(list.all().iter().all(|item| item.has_tag("bulk-tagged")));
}

#[test]
fn test_partition_by_priority() {
    let mut list = TodoList::new();
    list.add_with_priority("H1", Priority::High);
    list.add_with_priority("M1", Priority::Medium);
    list.add_with_priority("L1", Priority::Low);
    list.add_with_priority("H2", Priority::High);
    let (high, med, low) = list.partition_by_priority();
    assert_eq!(high.len(), 2);
    assert_eq!(med.len(), 1);
    assert_eq!(low.len(), 1);
}

// =====================================================================
// Edge Cases
// =====================================================================

#[test]
fn test_empty_list_operations() {
    let list = TodoList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.completed().len(), 0);
    assert_eq!(list.pending().len(), 0);
    assert_eq!(list.completed_count(), 0);
    assert_eq!(list.pending_count(), 0);
    assert!(list.ids().is_empty());
    assert!(list.titles().is_empty());
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
    assert_eq!(list.search("").len(), 1);
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

// =====================================================================
// Integration / Cross-topic tests
// =====================================================================

#[test]
fn test_full_workflow() {
    let mut list = TodoList::new();
    let id1 = list.add_with_priority("Write code", Priority::High);
    let id2 = list.add_with_tags("Review PR", &["work"]);
    let id3 = list.add("Buy groceries");
    list.toggle(id1);
    list.update_title(id3, "Buy organic groceries");
    list.get_mut(id2).unwrap().add_tag("urgent");

    assert_eq!(list.completed_count(), 1);
    assert_eq!(list.pending_count(), 2);
    assert_eq!(list.search("buy").len(), 1);
    assert_eq!(list.by_any_tag(&["work", "urgent"]).len(), 1);

    let stats = list.stats();
    assert_eq!(stats.total, 3);
    assert_eq!(stats.high_priority, 1);
}

#[test]
fn test_bulk_then_filter() {
    let mut list = TodoList::new();
    let id1 = list.add("A");
    let id2 = list.add("B");
    let id3 = list.add("C");
    list.set_priority_bulk(&[id1, id2, id3], Priority::High);
    list.toggle(id2);

    let pending_high = list.pending_by_priority(Priority::High);
    assert_eq!(pending_high.len(), 2);
}

#[test]
fn test_serialize_after_merge() {
    let mut list1 = TodoList::new();
    list1.add_with_priority("Task A", Priority::High);
    let mut list2 = TodoList::new();
    list2.add("Task B");
    list1.merge(list2);

    let serialized = list1.to_string_repr();
    let restored = TodoList::from_string_repr(&serialized);
    assert_eq!(restored.len(), 2);
    assert_eq!(restored.titles(), vec!["Task A", "Task B"]);
}

#[test]
fn test_stats_after_bulk_ops() {
    let mut list = TodoList::new();
    list.add("A");
    list.add("B");
    list.add("C");
    list.complete_all();
    let cleared = list.clear_completed();
    assert_eq!(cleared, 3);
    assert!(list.is_empty());
    assert_eq!(list.stats().total, 0);
}
