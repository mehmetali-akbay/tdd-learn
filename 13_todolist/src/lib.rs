// ============================================
// Level 4, Project 2: Todo List — Structs & Data Management
// Learn: CRUD, filtering, sorting, serialization, IDs
// ============================================

use std::fmt;

// ============================================
// Topic 1: TodoItem — Struct Design
// Learn: Struct with multiple fields, Display, equality
// ============================================

/// Priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
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

    /// Add a new item, returning its ID
    pub fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(TodoItem::new(id, title));
        id
    }

    /// Add an item with priority
    pub fn add_with_priority(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items
            .push(TodoItem::new(id, title).with_priority(priority));
        id
    }

    /// Get an item by ID
    pub fn get(&self, id: u32) -> Option<&TodoItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get a mutable reference by ID
    pub fn get_mut(&mut self, id: u32) -> Option<&mut TodoItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Remove an item by ID, returning it
    pub fn remove(&mut self, id: u32) -> Option<TodoItem> {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Toggle completion status of an item
    pub fn toggle(&mut self, id: u32) -> bool {
        if let Some(item) = self.get_mut(id) {
            item.toggle();
            true
        } else {
            false
        }
    }

    /// Total number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get all items as a slice
    pub fn all(&self) -> &[TodoItem] {
        &self.items
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Topic 3: Filtering
// Learn: Iterator-based filtering, closures
// ============================================

impl TodoList {
    /// Return completed items
    pub fn completed(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.completed).collect()
    }

    /// Return pending (not completed) items
    pub fn pending(&self) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| !item.completed).collect()
    }

    /// Filter items by priority
    pub fn by_priority(&self, priority: Priority) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| item.priority == priority)
            .collect()
    }

    /// Filter items by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&TodoItem> {
        self.items.iter().filter(|item| item.has_tag(tag)).collect()
    }

    /// Search items by title (case-insensitive)
    pub fn search(&self, query: &str) -> Vec<&TodoItem> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.title.to_lowercase().contains(&q))
            .collect()
    }

    /// Count of completed items
    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.completed).count()
    }

    /// Count of pending items
    pub fn pending_count(&self) -> usize {
        self.items.iter().filter(|item| !item.completed).count()
    }
}

// ============================================
// Topic 4: Sorting
// Learn: sort_by, Ord, multi-key sorting
// ============================================

impl TodoList {
    /// Return items sorted by priority (High first)
    pub fn sorted_by_priority(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| b.priority.cmp(&a.priority));
        items
    }

    /// Return items sorted by title alphabetically
    pub fn sorted_by_title(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| a.title.cmp(&b.title));
        items
    }

    /// Return items sorted: pending first, then by priority (High first)
    pub fn sorted_by_status_priority(&self) -> Vec<&TodoItem> {
        let mut items: Vec<&TodoItem> = self.items.iter().collect();
        items.sort_by(|a, b| {
            a.completed
                .cmp(&b.completed)
                .then(b.priority.cmp(&a.priority))
        });
        items
    }
}

// ============================================
// Topic 5: Serialization — to/from string
// Learn: Custom serialization (simple text format)
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

    /// Deserialize from a line
    pub fn from_line(line: &str) -> Option<TodoItem> {
        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if parts.len() != 5 {
            return None;
        }
        let id = parts[0].parse().ok()?;
        let title = parts[1].to_string();
        let completed = parts[2].parse().ok()?;
        let priority = match parts[3] {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => return None,
        };
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
    /// Serialize the entire list to a string
    pub fn to_string_repr(&self) -> String {
        self.items
            .iter()
            .map(|item| item.to_line())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Deserialize a list from a string
    pub fn from_string_repr(s: &str) -> Self {
        let items: Vec<TodoItem> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(TodoItem::from_line)
            .collect();
        let next_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        TodoList { items, next_id }
    }
}

// ============================================
// Topic 6: Advanced — Statistics & Bulk Operations
// Learn: Aggregation, batch operations
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
    /// Compute summary statistics
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

    /// Complete all items
    pub fn complete_all(&mut self) {
        for item in &mut self.items {
            item.completed = true;
        }
    }

    /// Remove all completed items, return their count
    pub fn clear_completed(&mut self) -> usize {
        let before = self.items.len();
        self.items.retain(|item| !item.completed);
        before - self.items.len()
    }

    /// Set priority for multiple items at once
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
}
