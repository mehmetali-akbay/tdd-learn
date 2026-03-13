#![allow(dead_code, unused_variables)]

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
        match self {
            ChatMessage::Join { username, room } => {
                format!("JOIN|{}|{}", username, room)
            }
            ChatMessage::Leave { username, room } => {
                format!("LEAVE|{}|{}", username, room)
            }
            ChatMessage::Text {
                username,
                room,
                content,
            } => {
                format!("TEXT|{}|{}|{}", username, room, content)
            }
            ChatMessage::PrivateMessage { from, to, content } => {
                format!("PM|{}|{}|{}", from, to, content)
            }
            ChatMessage::ServerNotice(msg) => {
                format!("NOTICE|{}", msg)
            }
        }
    }

    /// Parse a message from wire format.
    pub fn from_wire(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.splitn(4, '|').collect();
        match parts.first().copied() {
            Some("JOIN") if parts.len() >= 3 => Ok(ChatMessage::Join {
                username: parts[1].to_string(),
                room: parts[2].to_string(),
            }),
            Some("LEAVE") if parts.len() >= 3 => Ok(ChatMessage::Leave {
                username: parts[1].to_string(),
                room: parts[2].to_string(),
            }),
            Some("TEXT") if parts.len() >= 4 => Ok(ChatMessage::Text {
                username: parts[1].to_string(),
                room: parts[2].to_string(),
                content: parts[3].to_string(),
            }),
            Some("PM") if parts.len() >= 4 => Ok(ChatMessage::PrivateMessage {
                from: parts[1].to_string(),
                to: parts[2].to_string(),
                content: parts[3].to_string(),
            }),
            Some("NOTICE") if parts.len() >= 2 => {
                Ok(ChatMessage::ServerNotice(parts[1].to_string()))
            }
            _ => Err(format!("invalid wire format: '{}'", s)),
        }
    }

    /// Get the room this message belongs to, if any.
    pub fn room(&self) -> Option<&str> {
        match self {
            ChatMessage::Join { room, .. }
            | ChatMessage::Leave { room, .. }
            | ChatMessage::Text { room, .. } => Some(room),
            _ => None,
        }
    }

    /// Get the sender username, if any.
    pub fn sender(&self) -> Option<&str> {
        match self {
            ChatMessage::Join { username, .. }
            | ChatMessage::Leave { username, .. }
            | ChatMessage::Text { username, .. } => Some(username),
            ChatMessage::PrivateMessage { from, .. } => Some(from),
            ChatMessage::ServerNotice(_) => None,
        }
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
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Get a receiver that will be notified on shutdown.
    pub fn subscribe(&self) -> broadcast::Receiver<ChatMessage> {
        self.sender.subscribe()
    }

    /// Broadcast a message to all subscribers.
    pub fn broadcast(&self, msg: ChatMessage) -> Result<usize, String> {
        self.sender
            .send(msg)
            .map_err(|e| format!("broadcast failed: {}", e))
    }
}

// ============================================
// Topic 3: Session Management
// Learn: Track connected users, nicknames
// ============================================

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
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Register a new user. Returns Err if username is taken.
    pub fn connect(&mut self, username: &str) -> Result<(), String> {
        if self.sessions.contains_key(username) {
            return Err(format!("username '{}' already taken", username));
        }
        self.sessions.insert(
            username.to_string(),
            Session {
                username: username.to_string(),
                current_room: None,
            },
        );
        Ok(())
    }

    /// Remove a user.
    pub fn disconnect(&mut self, username: &str) -> Option<Session> {
        self.sessions.remove(username)
    }

    /// Check if a user is connected.
    pub fn is_connected(&self, username: &str) -> bool {
        self.sessions.contains_key(username)
    }

    /// Get a user's session.
    pub fn get_session(&self, username: &str) -> Option<&Session> {
        self.sessions.get(username)
    }

    /// Set the user's current room.
    pub fn set_room(&mut self, username: &str, room: &str) -> Result<(), String> {
        let session = self
            .sessions
            .get_mut(username)
            .ok_or_else(|| format!("user '{}' not connected", username))?;
        session.current_room = Some(room.to_string());
        Ok(())
    }

    /// List all connected usernames.
    pub fn connected_users(&self) -> Vec<String> {
        self.sessions.keys().cloned().collect()
    }

    /// Count connected users.
    pub fn count(&self) -> usize {
        self.sessions.len()
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
        Self {
            name: name.to_string(),
            members: HashSet::new(),
        }
    }

    pub fn join(&mut self, username: &str) -> bool {
        self.members.insert(username.to_string())
    }

    pub fn leave(&mut self, username: &str) -> bool {
        self.members.remove(username)
    }

    pub fn is_member(&self, username: &str) -> bool {
        self.members.contains(username)
    }

    pub fn members(&self) -> Vec<String> {
        self.members.iter().cloned().collect()
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// Room manager handles all chat rooms.
pub struct RoomManager {
    rooms: HashMap<String, Room>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    /// Create a new room. Returns false if it already exists.
    pub fn create_room(&mut self, name: &str) -> bool {
        if self.rooms.contains_key(name) {
            return false;
        }
        self.rooms.insert(name.to_string(), Room::new(name));
        true
    }

    /// Join a room (creates it if it doesn't exist).
    pub fn join_room(&mut self, room_name: &str, username: &str) -> bool {
        let room = self
            .rooms
            .entry(room_name.to_string())
            .or_insert_with(|| Room::new(room_name));
        room.join(username)
    }

    /// Leave a room. Removes room if empty.
    pub fn leave_room(&mut self, room_name: &str, username: &str) -> bool {
        if let Some(room) = self.rooms.get_mut(room_name) {
            let removed = room.leave(username);
            if room.member_count() == 0 {
                self.rooms.remove(room_name);
            }
            removed
        } else {
            false
        }
    }

    /// Get members of a room.
    pub fn room_members(&self, room_name: &str) -> Option<Vec<String>> {
        self.rooms.get(room_name).map(|r| r.members())
    }

    /// List all room names.
    pub fn list_rooms(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    /// Remove a user from all rooms they're in.
    pub fn remove_user_from_all(&mut self, username: &str) -> Vec<String> {
        let room_names: Vec<String> = self
            .rooms
            .iter()
            .filter(|(_, room)| room.is_member(username))
            .map(|(name, _)| name.clone())
            .collect();

        for name in &room_names {
            self.leave_room(name, username);
        }

        room_names
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
        let (sender, _) = broadcast::channel(1);
        Self { sender }
    }

    /// Get a receiver that will be notified on shutdown.
    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.sender.subscribe()
    }

    /// Trigger shutdown.
    pub fn shutdown(&self) {
        let _ = self.sender.send(());
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
        Self {
            sessions: SessionManager::new(),
            rooms: RoomManager::new(),
            hub: MessageHub::new(256),
        }
    }

    /// A user connects.
    pub fn user_connect(&mut self, username: &str) -> Result<(), String> {
        self.sessions.connect(username)
    }

    /// A user disconnects.
    pub fn user_disconnect(&mut self, username: &str) -> Vec<ChatMessage> {
        let rooms = self.rooms.remove_user_from_all(username);
        self.sessions.disconnect(username);
        rooms
            .into_iter()
            .map(|room| ChatMessage::Leave {
                username: username.to_string(),
                room,
            })
            .collect()
    }

    /// A user joins a room.
    pub fn user_join_room(&mut self, username: &str, room: &str) -> Result<ChatMessage, String> {
        if !self.sessions.is_connected(username) {
            return Err(format!("user '{}' not connected", username));
        }
        self.rooms.join_room(room, username);
        self.sessions.set_room(username, room)?;
        let msg = ChatMessage::Join {
            username: username.to_string(),
            room: room.to_string(),
        };
        let _ = self.hub.broadcast(msg.clone());
        Ok(msg)
    }

    /// A user sends a message to their current room.
    pub fn user_send(&mut self, username: &str, content: &str) -> Result<ChatMessage, String> {
        let session = self
            .sessions
            .get_session(username)
            .ok_or_else(|| format!("user '{}' not connected", username))?;
        let room = session
            .current_room
            .clone()
            .ok_or_else(|| format!("user '{}' not in any room", username))?;
        let msg = ChatMessage::Text {
            username: username.to_string(),
            room,
            content: content.to_string(),
        };
        let _ = self.hub.broadcast(msg.clone());
        Ok(msg)
    }

    /// Get all users in a room.
    pub fn room_users(&self, room: &str) -> Option<Vec<String>> {
        self.rooms.room_members(room)
    }
}
