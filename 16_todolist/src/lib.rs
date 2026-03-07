// ============================================
// Level 4, Project 2: Todo List — Structs & Data Management
// Learn: CRUD, filtering, sorting, serialization, IDs,
//        builder patterns, bulk ops, iteration, transformation
// ============================================

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

// ============================================
// Topic 1: TodoItem — Struct Design
// Learn: Struct with multiple fields, Display, equality, builder pattern
// Reinforces: structs (field design), enums (Priority), patterns (matching)
// ============================================

/// Priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

/// Parse a priority from a string.
impl Priority {
    pub fn from_str_opt(s: &str) -> Option<Priority> {
        match s {
            "low" => Some(Priority::Low),
            "medium" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            _ => None,
        }
    }
}

/// A single to-do item
#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub priority: Priority,
    pub tags: Vec<String>,
}

impl TodoItem {
    pub fn new(id: u32, title: &str) -> Self {
        TodoItem {
            id,
            title: title.to_string(),
            completed: false,
            priority: Priority::Medium,
            tags: Vec::new(),
        }
    }

    /// Builder: set priority.
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Builder: set tags.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Toggle completion status.
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    /// Check if the item has a specific tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    /// Add a tag (no duplicates).
    pub fn add_tag(&mut self, tag: &str) {
        if !self.has_tag(tag) {
            self.tags.push(tag.to_string());
        }
    }

    /// Remove a tag, return whether it was found.
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            true
        } else {
            false
        }
    }

    /// Rename the item.
    pub fn rename(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    /// Remove all tags.
    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    /// Check if this item is high priority.
    pub fn is_high_priority(&self) -> bool {
        self.priority == Priority::High
    }

    /// Number of tags.
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { "☐" };
        write!(
            f,
            "[{}] #{}: {} ({})",
            status, self.id, self.title, self.priority
        )
    }
}

// ============================================
// Topic 2: TodoList — Collection Management
// Learn: Vec-backed collection, CRUD, ID generation
// Reinforces: vecs (Vec operations), results (Option)
// ============================================

/// A list of to-do items
#[derive(Debug, Clone)]
pub struct TodoList {
    items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a new item, returning its ID.
    pub fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(TodoItem::new(id, title));
        id
    }

    /// Add an item with priority.
    pub fn add_with_priority(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items
            .push(TodoItem::new(id, title).with_priority(priority));
        id
    }

    /// Add an item with tags.
    pub fn add_with_tags(&mut self, title: &str, tags: &[&str]) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items
            .push(TodoItem::new(id, title).with_tags(tags));
        id
    }

    /// Get an item by ID.
    pub fn get(&self, id: u32) -> Option<&TodoItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get a mutable reference by ID.
    pub fn get_mut(&mut self, id: u32) -> Option<&mut TodoItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Remove an item by ID, returning it.
    pub fn remove(&mut self, id: u32) -> Option<TodoItem> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Toggle completion status of an item.
    pub fn toggle(&mut self, id: u32) -> bool {
        if let Some(item) = self.get_mut(id) {
            item.toggle();
            true
        } else {
            false
        }
    }

    /// Update the title of an item. Returns true if found.
    pub fn update_title(&mut self, id: u32, title: &str) -> bool {
        if let Some(item) = self.get_mut(id) {
            item.rename(title);
            true
        } else {
            false
        }
    }

    /// Check if an item with this ID exists.
    pub fn contains(&self, id: u32) -> bool {
        self.items.iter().any(|item| item.id == id)
    }

    /// Total number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get all items as a slice.
    pub fn all(&self) -> &[TodoItem] {
        &self.items
    }

    /// Collect all item IDs.
    pub fn ids(&self) -> Vec<u32> {
        self.items.iter().map(|item| item.id).collect()
    }

    /// Collect all titles as string references.
    pub fn titles(&self) -> Vec<&str> {
        self.items.iter().map(|item| item.title.as_str()).collect()
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 3: Filtering & Searching
// Learn: Iterator-based filtering, closures, combined predicates
// Reinforces: iterators (filter, map), closures (Fn trait)
// ============================================

impl TodoList {
    /// Return completed items.
    pub fn completed(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.completed).collect()
    }

    /// Return pending (not completed) items.
    pub fn pending(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    /// Filter items by priority.
    pub fn by_priority(&self, priority: Priority) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| item.priority == priority)
            .collect()
    }

    /// Filter items by tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.has_tag(tag)).collect()
    }

    /// Filter items matching any of the given tags.
    pub fn by_any_tag(&self, tags: &[&str]) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| tags.iter().any(|&t| item.has_tag(t)))
            .collect()
    }

    /// Search items by title (case-insensitive substring).
    pub fn search(&self, query: &str) -> Vec<&TodoItem> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.title.to_lowercase().contains(&q))
            .collect()
    }

    /// Generic filter with a custom predicate.
    pub fn filter_by(&self, pred: impl Fn(&TodoItem) -> bool) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| pred(item)).collect()
    }

    /// Find the first item matching a predicate.
    pub fn find_first(&self, pred: impl Fn(&TodoItem) -> bool) -> Option<&TodoItem> {
        self.items.iter().find(|item| pred(item))
    }

    /// Pending items of a specific priority.
    pub fn pending_by_priority(&self, priority: Priority) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| !item.completed && item.priority == priority)
            .collect()
    }

    /// Count of completed items.
    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.completed).count()
    }

    /// Count of pending items.
    pub fn pending_count(&self) -> usize {
        self.items.iter().filter(|item| !item.completed).count()
    }
}

// ============================================
// Topic 4: Sorting & Ordering
// Learn: sort_by, Ord, multi-key sorting, custom comparators
// Reinforces: iterators (sort), patterns (matching), closures
// ============================================

impl TodoList {
    /// Return items sorted by priority (High first).
    pub fn sorted_by_priority(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| b.priority.cmp(&a.priority));
        items
    }

    /// Return items sorted by title alphabetically.
    pub fn sorted_by_title(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| a.title.cmp(&b.title));
        items
    }

    /// Return items sorted: pending first, then by priority (High first).
    pub fn sorted_by_status_priority(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| {
            a.completed
                .cmp(&b.completed)
                .then(b.priority.cmp(&a.priority))
        });
        items
    }

    /// Return items sorted by ID.
    pub fn sorted_by_id(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by_key(|item| item.id);
        items
    }

    /// Return items sorted by tag count (most tags first).
    pub fn sorted_by_tag_count(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by_key(|b| std::cmp::Reverse(b.tag_count()));
        items
    }

    /// Sort with a user-supplied comparator.
    pub fn sorted_by_custom(
        &self,
        mut cmp: impl FnMut(&TodoItem, &TodoItem) -> std::cmp::Ordering,
    ) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| cmp(a, b));
        items
    }
}

// ============================================
// Topic 5: Serialization — to/from string
// Learn: Custom serialization (simple text format), formatting
// Reinforces: strings (parsing, formatting), results (Option/Result)
// ============================================

impl TodoItem {
    /// Serialize to a line: "id|title|completed|priority|tag1,tag2"
    pub fn to_line(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.id,
            self.title,
            self.completed,
            self.priority,
            self.tags.join(",")
        )
    }

    /// Deserialize from a line.
    pub fn from_line(line: &str) -> Option<TodoItem> {
        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if parts.len() != 5 {
            return None;
        }
        let id = parts[0].parse().ok()?;
        let title = parts[1].to_string();
        let completed = parts[2].parse().ok()?;
        let priority = Priority::from_str_opt(parts[3])?;
        let tags = if parts[4].is_empty() {
            vec![]
        } else {
            parts[4].split(',').map(|s| s.to_string()).collect()
        };
        Some(TodoItem {
            id,
            title,
            completed,
            priority,
            tags,
        })
    }
}

impl TodoList {
    /// Serialize the entire list to a string.
    pub fn to_string_repr(&self) -> String {
        self.items
            .iter()
            .map(|item| item.to_line())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Deserialize a list from a string.
    pub fn from_string_repr(s: &str) -> Self {
        let items: Vec<TodoItem> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(TodoItem::from_line)
            .collect();
        let next_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        TodoList { items, next_id }
    }

    /// Human-readable summary of the list.
    pub fn to_summary(&self) -> String {
        let stats = self.stats();
        let mut lines = vec![format!(
            "Todo List: {} items ({} completed, {} pending)",
            stats.total, stats.completed, stats.pending
        )];
        if stats.total > 0 {
            lines.push(format!(
                "Completion: {:.0}%",
                stats.completion_rate * 100.0
            ));
        }
        for item in &self.items {
            lines.push(format!("  {}", item));
        }
        lines.join("\n")
    }
}

// ============================================
// Topic 6: Advanced — Statistics & Bulk Operations
// Learn: Aggregation, batch operations, HashMap counting
// Reinforces: hashmaps (counting), iterators (fold, collect)
// ============================================

/// Summary statistics for a TodoList
#[derive(Debug, PartialEq)]
pub struct Stats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub high_priority: usize,
    pub completion_rate: f64,
}

impl TodoList {
    /// Compute summary statistics.
    pub fn stats(&self) -> Stats {
        let total = self.items.len();
        let completed = self.completed_count();
        Stats {
            total,
            completed,
            pending: total - completed,
            high_priority: self.by_priority(Priority::High).len(),
            completion_rate: if total > 0 {
                completed as f64 / total as f64
            } else {
                0.0
            },
        }
    }

    /// Complete all items.
    pub fn complete_all(&mut self) {
        for item in &mut self.items {
            item.completed = true;
        }
    }

    /// Uncomplete all items (reset to pending).
    pub fn uncomplete_all(&mut self) {
        for item in &mut self.items {
            item.completed = false;
        }
    }

    /// Remove all completed items, return their count.
    pub fn clear_completed(&mut self) -> usize {
        let before = self.items.len();
        self.items.retain(|item| !item.completed);
        before - self.items.len()
    }

    /// Set priority for multiple items at once.
    pub fn set_priority_bulk(&mut self, ids: &[u32], priority: Priority) -> usize {
        let mut count = 0;
        for item in &mut self.items {
            if ids.contains(&item.id) {
                item.priority = priority;
                count += 1;
            }
        }
        count
    }

    /// Add a tag to multiple items at once.
    pub fn add_tag_bulk(&mut self, ids: &[u32], tag: &str) -> usize {
        let mut count = 0;
        for item in &mut self.items {
            if ids.contains(&item.id) {
                item.add_tag(tag);
                count += 1;
            }
        }
        count
    }

    /// Remove all items of a given priority, return count removed.
    pub fn remove_by_priority(&mut self, priority: Priority) -> usize {
        let before = self.items.len();
        self.items.retain(|item| item.priority != priority);
        before - self.items.len()
    }

    /// Count items per priority level.
    pub fn priority_distribution(&self) -> HashMap<Priority, usize> {
        let mut dist = HashMap::new();
        for item in &self.items {
            *dist.entry(item.priority).or_insert(0) += 1;
        }
        dist
    }
}

// ============================================
// Topic 7: Iteration & Transformation
// Learn: Custom iteration, merging, transforming collections
// Reinforces: iterators (adapters, collect), closures (FnMut)
// ============================================

impl TodoList {
    /// Iterate over items.
    pub fn iter(&self) -> std::slice::Iter<'_, TodoItem> {
        self.items.iter()
    }

    /// Consume the list and return the inner items.
    pub fn into_items(self) -> Vec<TodoItem> {
        self.items
    }

    /// Merge another list into this one (re-assigns IDs).
    pub fn merge(&mut self, other: TodoList) {
        for mut item in other.items {
            item.id = self.next_id;
            self.next_id += 1;
            self.items.push(item);
        }
    }

    /// Map all titles through a function, returning the new strings.
    pub fn map_titles(&self, f: impl Fn(&str) -> String) -> Vec<String> {
        self.items.iter().map(|item| f(&item.title)).collect()
    }

    /// Collect all unique tags (sorted) across every item.
    pub fn all_tags(&self) -> Vec<String> {
        let set: HashSet<&str> = self
            .items
            .iter()
            .flat_map(|item| item.tags.iter().map(|t| t.as_str()))
            .collect();
        let mut tags: Vec<String> = set.into_iter().map(|s| s.to_string()).collect();
        tags.sort();
        tags
    }

    /// Find the most frequently used tag, if any.
    pub fn most_common_tag(&self) -> Option<String> {
        let mut freq: HashMap<&str, usize> = HashMap::new();
        for item in &self.items {
            for tag in &item.tags {
                *freq.entry(tag.as_str()).or_insert(0) += 1;
            }
        }
        freq.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tag, _)| tag.to_string())
    }

    /// Apply a mutation to every item.
    pub fn apply_to_all(&mut self, mut f: impl FnMut(&mut TodoItem)) {
        for item in &mut self.items {
            f(item);
        }
    }

    /// Split items into (high, medium, low) priority groups.
    pub fn partition_by_priority(
        &self,
    ) -> (Vec<&TodoItem>, Vec<&TodoItem>, Vec<&TodoItem>) {
        let high = self.by_priority(Priority::High);
        let medium = self.by_priority(Priority::Medium);
        let low = self.by_priority(Priority::Low);
        (high, medium, low)
    }
}
