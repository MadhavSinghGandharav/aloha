# 🗨️ Aloha - Terminal-Based Chatroom in Rust

**Aloha** is a fast, lightweight, and concurrent terminal-based chatroom built with Rust. It allows multiple clients to connect to a central server and exchange real-time messages, with support for an admin user who can manage the room.

---

## 🚀 Features

- 🌐 Real-time chat over TCP
- 🧵 Threaded handling for multiple clients
- 📢 Message broadcasting to all connected users
- 🔒 Admin recognition (first/localhost client)
- 🦺 Graceful client disconnection handling
- 🛠️ Built in Rust using standard libraries and minimal external crates

---

## 📁 Project Structure

```bash
aloha/
├── Cargo.toml
├── src/
│   ├── main.rs            # Entry point
│   ├── client/            # Client-side logic
│   │   ├── connection.rs
│   │   └── handler.rs
│   ├── server/            # Server-side logic
│   │   ├── listener.rs
│   │   ├── handler.rs
│   │   └── room.rs
│   └── common/            # Shared utilities
│       ├── client.rs
│       └── utils.rs
└── README.md
