// ============================================
// Level 9, Project 5: Async Chat Server — Capstone
// Learn: TCP async server, broadcast, rooms, graceful shutdown
// ============================================

use std::collections::{HashMap, HashSet};
use tokio::sync::broadcast;

// ============================================
// Topic 1: Message Protocol
// Learn: Define and parse chat message types
// ============================================

/// Chat message types.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatMessage {
    Join {
        username: String,
        room: String,
    },
    Leave {
        username: String,
        room: String,
    },
    Text {
        username: String,
        room: String,
        content: String,
    },
    PrivateMessage {
        from: String,
        to: String,
        content: String,
    },
    ServerNotice(String),
}

impl ChatMessage {
    /// Serialize a message to a wire format: "TYPE|field1|field2|..."
    pub fn to_wire(&self) -> String {
        todo!()
    }

    /// Parse a message from wire format.
    pub fn from_wire(s: &str) -> Result<Self, String> {
        todo!()
    }

    /// Get the room this message belongs to, if any.
    pub fn room(&self) -> Option<&str> {
        todo!()
    }

    /// Get the sender username, if any.
    pub fn sender(&self) -> Option<&str> {
        todo!()
    }
}

// ============================================
// Topic 2: Broadcast Channel
// Learn: tokio::sync::broadcast for message fanout
// ============================================

/// A simple message hub that broadcasts to all subscribers.
pub struct MessageHub {
    sender: broadcast::Sender<ChatMessage>,
}

impl MessageHub {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Subscribe to receive messages.
    pub fn subscribe(&self) -> broadcast::Receiver<ChatMessage> {
        todo!()
    }

    /// Broadcast a message to all subscribers.
    pub fn broadcast(&self, msg: ChatMessage) -> Result<usize, String> {
        todo!()
    }
}

// ============================================
// Topic 3: Session Management
// Learn: Track connected users, nicknames
// ============================================

/// A user session.
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub username: String,
    pub current_room: Option<String>,
}

/// Session manager tracks all connected users.
pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        todo!()
    }

    /// Register a new user. Returns Err if username is taken.
    pub fn connect(&mut self, username: &str) -> Result<(), String> {
        todo!()
    }

    /// Remove a user.
    pub fn disconnect(&mut self, username: &str) -> Option<Session> {
        todo!()
    }

    /// Check if a user is connected.
    pub fn is_connected(&self, username: &str) -> bool {
        todo!()
    }

    /// Get a user's session.
    pub fn get_session(&self, username: &str) -> Option<&Session> {
        todo!()
    }

    /// Set the user's current room.
    pub fn set_room(&mut self, username: &str, room: &str) -> Result<(), String> {
        todo!()
    }

    /// List all connected usernames.
    pub fn connected_users(&self) -> Vec<String> {
        todo!()
    }

    /// Count connected users.
    pub fn count(&self) -> usize {
        todo!()
    }
}

// ============================================
// Topic 4: Chat Room Logic
// Learn: Multiple rooms, join/leave
// ============================================

/// A chat room with members.
pub struct Room {
    pub name: String,
    members: HashSet<String>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        todo!()
    }

    pub fn join(&mut self, username: &str) -> bool {
        todo!()
    }

    pub fn leave(&mut self, username: &str) -> bool {
        todo!()
    }

    pub fn is_member(&self, username: &str) -> bool {
        todo!()
    }

    pub fn members(&self) -> Vec<String> {
        todo!()
    }

    pub fn member_count(&self) -> usize {
        todo!()
    }
}

/// Room manager handles all chat rooms.
pub struct RoomManager {
    rooms: HashMap<String, Room>,
}

impl RoomManager {
    pub fn new() -> Self {
        todo!()
    }

    /// Create a new room. Returns false if it already exists.
    pub fn create_room(&mut self, name: &str) -> bool {
        todo!()
    }

    /// Join a room (creates it if it doesn't exist).
    pub fn join_room(&mut self, room_name: &str, username: &str) -> bool {
        todo!()
    }

    /// Leave a room. Removes room if empty.
    pub fn leave_room(&mut self, room_name: &str, username: &str) -> bool {
        todo!()
    }

    /// Get members of a room.
    pub fn room_members(&self, room_name: &str) -> Option<Vec<String>> {
        todo!()
    }

    /// List all room names.
    pub fn list_rooms(&self) -> Vec<String> {
        todo!()
    }

    /// Remove a user from all rooms they're in.
    pub fn remove_user_from_all(&mut self, username: &str) -> Vec<String> {
        todo!()
    }
}

// ============================================
// Topic 5: Graceful Shutdown
// Learn: Shutdown coordination
// ============================================

/// A shutdown signal coordinator.
pub struct ShutdownSignal {
    sender: broadcast::Sender<()>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        todo!()
    }

    /// Get a receiver that will be notified on shutdown.
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        todo!()
    }

    /// Trigger shutdown.
    pub fn shutdown(&self) {
        todo!()
    }
}

// ============================================
// Topic 6: Advanced — Full Chat Server Integration
// Learn: End-to-end message flow
// ============================================

/// The main chat server that ties everything together.
pub struct ChatServer {
    pub sessions: SessionManager,
    pub rooms: RoomManager,
    pub hub: MessageHub,
}

impl ChatServer {
    pub fn new() -> Self {
        todo!()
    }

    /// A user connects.
    pub fn user_connect(&mut self, username: &str) -> Result<(), String> {
        todo!()
    }

    /// A user disconnects.
    pub fn user_disconnect(&mut self, username: &str) -> Vec<ChatMessage> {
        todo!()
    }

    /// A user joins a room.
    pub fn user_join_room(&mut self, username: &str, room: &str) -> Result<ChatMessage, String> {
        todo!()
    }

    /// A user sends a message to their current room.
    pub fn user_send(&mut self, username: &str, content: &str) -> Result<ChatMessage, String> {
        todo!()
    }

    /// Get all users in a room.
    pub fn room_users(&self, room: &str) -> Option<Vec<String>> {
        todo!()
    }
}

// ============================================
// Tests
// ============================================
