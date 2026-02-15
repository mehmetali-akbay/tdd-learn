use async_project::*;

// ===== Topic 1: Message Protocol =====

#[test]
fn test_message_to_wire_join() {
    let msg = ChatMessage::Join {
        username: "alice".into(),
        room: "general".into(),
    };
    let wire = msg.to_wire();
    assert!(wire.contains("JOIN"));
    assert!(wire.contains("alice"));
    assert!(wire.contains("general"));
}

#[test]
fn test_message_roundtrip() {
    let msg = ChatMessage::Text {
        username: "bob".into(),
        room: "general".into(),
        content: "hello".into(),
    };
    let wire = msg.to_wire();
    let parsed = ChatMessage::from_wire(&wire).unwrap();
    assert_eq!(msg, parsed);
}

#[test]
fn test_message_from_wire_invalid() {
    assert!(ChatMessage::from_wire("INVALID").is_err());
}

#[test]
fn test_message_room() {
    let msg = ChatMessage::Text {
        username: "a".into(),
        room: "lobby".into(),
        content: "hi".into(),
    };
    assert_eq!(msg.room(), Some("lobby"));
}

#[test]
fn test_message_sender() {
    let msg = ChatMessage::ServerNotice("notice".into());
    assert_eq!(msg.sender(), None);
}

// ===== Topic 2: Broadcast =====

#[tokio::test]
async fn test_broadcast_basic() {
    let hub = MessageHub::new(16);
    let mut rx = hub.subscribe();
    let msg = ChatMessage::ServerNotice("hello".into());
    hub.broadcast(msg.clone()).unwrap();
    let received = rx.recv().await.unwrap();
    assert_eq!(received, msg);
}

#[tokio::test]
async fn test_broadcast_multiple_subscribers() {
    let hub = MessageHub::new(16);
    let mut rx1 = hub.subscribe();
    let mut rx2 = hub.subscribe();
    let msg = ChatMessage::ServerNotice("test".into());
    hub.broadcast(msg.clone()).unwrap();
    assert_eq!(rx1.recv().await.unwrap(), msg);
    assert_eq!(rx2.recv().await.unwrap(), msg);
}

// ===== Topic 3: Session Management =====

#[test]
fn test_session_connect() {
    let mut sm = SessionManager::new();
    assert!(sm.connect("alice").is_ok());
    assert!(sm.is_connected("alice"));
    assert_eq!(sm.count(), 1);
}

#[test]
fn test_session_duplicate() {
    let mut sm = SessionManager::new();
    sm.connect("alice").unwrap();
    assert!(sm.connect("alice").is_err());
}

#[test]
fn test_session_disconnect() {
    let mut sm = SessionManager::new();
    sm.connect("alice").unwrap();
    let session = sm.disconnect("alice").unwrap();
    assert_eq!(session.username, "alice");
    assert!(!sm.is_connected("alice"));
}

#[test]
fn test_session_set_room() {
    let mut sm = SessionManager::new();
    sm.connect("alice").unwrap();
    sm.set_room("alice", "general").unwrap();
    let session = sm.get_session("alice").unwrap();
    assert_eq!(session.current_room, Some("general".into()));
}

// ===== Topic 4: Rooms =====

#[test]
fn test_room_join_leave() {
    let mut room = Room::new("general");
    assert!(room.join("alice"));
    assert!(room.is_member("alice"));
    assert_eq!(room.member_count(), 1);
    assert!(room.leave("alice"));
    assert!(!room.is_member("alice"));
}

#[test]
fn test_room_duplicate_join() {
    let mut room = Room::new("general");
    assert!(room.join("alice"));
    assert!(!room.join("alice"));
}

#[test]
fn test_room_manager() {
    let mut rm = RoomManager::new();
    rm.join_room("general", "alice");
    rm.join_room("general", "bob");
    rm.join_room("random", "alice");

    let members = rm.room_members("general").unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(rm.list_rooms().len(), 2);
}

#[test]
fn test_room_manager_remove_user() {
    let mut rm = RoomManager::new();
    rm.join_room("a", "alice");
    rm.join_room("b", "alice");
    let rooms = rm.remove_user_from_all("alice");
    assert_eq!(rooms.len(), 2);
}

// ===== Topic 5: Shutdown =====

#[tokio::test]
async fn test_shutdown_signal() {
    let signal = ShutdownSignal::new();
    let mut rx = signal.subscribe();
    signal.shutdown();
    assert!(rx.recv().await.is_ok());
}

// ===== Topic 6: Full Server =====

#[test]
fn test_chat_server_connect() {
    let mut server = ChatServer::new();
    assert!(server.user_connect("alice").is_ok());
    assert!(server.user_connect("alice").is_err());
}

#[test]
fn test_chat_server_join_room() {
    let mut server = ChatServer::new();
    server.user_connect("alice").unwrap();
    let msg = server.user_join_room("alice", "general").unwrap();
    assert!(matches!(msg, ChatMessage::Join { .. }));

    let users = server.room_users("general").unwrap();
    assert!(users.contains(&"alice".to_string()));
}

#[test]
fn test_chat_server_send() {
    let mut server = ChatServer::new();
    server.user_connect("alice").unwrap();
    server.user_join_room("alice", "general").unwrap();
    let msg = server.user_send("alice", "hello").unwrap();
    assert!(matches!(msg, ChatMessage::Text { .. }));
}

#[test]
fn test_chat_server_disconnect() {
    let mut server = ChatServer::new();
    server.user_connect("alice").unwrap();
    server.user_join_room("alice", "general").unwrap();
    let messages = server.user_disconnect("alice");
    assert!(!messages.is_empty());
}

// ===== Edge Cases =====

#[test]
fn test_message_leave_roundtrip() {
    let msg = ChatMessage::Leave {
        username: "bob".into(),
        room: "general".into(),
    };
    let wire = msg.to_wire();
    assert_eq!(ChatMessage::from_wire(&wire).unwrap(), msg);
}

#[test]
fn test_message_private_roundtrip() {
    let msg = ChatMessage::PrivateMessage {
        from: "alice".into(),
        to: "bob".into(),
        content: "secret".into(),
    };
    let wire = msg.to_wire();
    assert_eq!(ChatMessage::from_wire(&wire).unwrap(), msg);
}

#[test]
fn test_notice_has_no_room_or_sender() {
    let msg = ChatMessage::ServerNotice("hello".into());
    assert_eq!(msg.room(), None);
    assert_eq!(msg.sender(), None);
}

#[test]
fn test_session_list_users() {
    let mut sm = SessionManager::new();
    sm.connect("alice").unwrap();
    sm.connect("bob").unwrap();
    let mut users = sm.connected_users();
    users.sort();
    assert_eq!(users, vec!["alice", "bob"]);
}

#[test]
fn test_session_disconnect_nonexistent() {
    let mut sm = SessionManager::new();
    assert!(sm.disconnect("ghost").is_none());
}

#[test]
fn test_room_leave_nonmember() {
    let mut room = Room::new("test");
    assert!(!room.leave("ghost"));
}

#[test]
fn test_room_manager_empty_room() {
    let rm = RoomManager::new();
    assert!(rm.room_members("nonexistent").is_none());
}

#[test]
fn test_server_send_not_connected() {
    let mut server = ChatServer::new();
    assert!(server.user_send("ghost", "hello").is_err());
}

#[test]
fn test_server_join_not_connected() {
    let mut server = ChatServer::new();
    assert!(server.user_join_room("ghost", "general").is_err());
}

#[test]
fn test_room_members_list() {
    let mut room = Room::new("test");
    room.join("alice");
    room.join("bob");
    let mut members = room.members();
    members.sort();
    assert_eq!(members, vec!["alice", "bob"]);
}
