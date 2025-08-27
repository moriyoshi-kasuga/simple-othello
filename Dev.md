# Simple Othello - Development Guide

This document outlines the technical details, architecture, and development workflow for the Simple Othello project. It is intended for both human developers and AI agents involved in the development process.

## 1. Project Overview

Simple Othello is a full-stack web application that allows users to play the game of Othello against each other in real-time. The entire application is written in Rust, utilizing a WebSocket-based communication protocol for low-latency interaction between the client and server.

- **Real-time Gameplay**: The client and server communicate over WebSockets.
- **Rust Full-stack**: Both the frontend and backend are implemented in Rust, enabling code sharing and type safety across the entire stack.
- **Minimal Dependencies**: The project aims to use a minimal set of well-established libraries.

## 2. Technology Stack

- **Backend**:
  - **Framework**: [Axum](https://github.com/tokio-rs/axum)
  - **Async Runtime**: [Tokio](https://tokio.rs/)
  - **WebSocket**: Axum's built-in WebSocket support.

- **Frontend**:
  - **Framework**: [Dioxus](https://dioxuslabs.com/)
  - **WebSocket Client**: [gloo-net](https://github.com/rustwasm/gloo) for WebSocket connections.
  - **Build Tool**: Dioxus CLI (`dx`) for managing the frontend build process (e.g., `dx serve`).
  - **Styling**: [Tailwind CSS](https://tailwindcss.com/) for styling.

- **Core & Shared Logic**:
  - **Game Logic**: A custom bitboard implementation for efficient Othello game state management.
  - **Serialization**: [Serde](https://serde.rs/) for data serialization/deserialization (JSON format).
  - **Unique IDs**: A custom `Uid` type based on [ULID](https://github.com/ulid/spec) for uniquely identifying entities.

## 3. Directory & Crate Structure

The project is organized as a Cargo workspace with several distinct crates, each having a specific responsibility.

```txt
/
├── client/         # Frontend Dioxus application
├── core/           # Core Othello game logic (bitboard)
├── extras/
│   └── uid/        # ULID wrapper for generating unique IDs
├── net/            # Shared network packet definitions for client-server communication
├── server/         # Backend Axum WebSocket server
├── Cargo.toml      # Workspace configuration
└── Dev.md          # This development guide
```

### Crate Details

- **`server`**:
  - The main entry point for the backend application (`server/src/main.rs`).
  - It initializes an Axum router, sets up a WebSocket endpoint at `/ws`, and manages the application state (`AppState`).
  - Handles WebSocket connections, passing each new socket to `handle::handle_socket` for processing.

- **`client`**:
  - The main entry point for the frontend WebAssembly application (`client/src/main.rs`).
  - It uses the Dioxus framework to create a single-page application (SPA).
  - The root `App` component provides the `AppState` context which contains the WebSocket `Connection`.
  - The UI is composed of various components located in `client/src/components`.
  - State management is handled in `client/src/state`, with the `Connection` struct managing WebSocket communication using `gloo-net`.
  - Styling is handled with Tailwind CSS, configured via `tailwind.css`.

- **`core`**:
  - Contains the `OthelloBoard` struct, which implements the game's rules using a bitboard representation (`u64`).
  - This approach is highly efficient for calculating legal moves, placing pieces, and flipping opponent pieces.
  - It is a self-contained library with no external dependencies.

- **`net`**:
  - Defines the communication protocol between the client and server.
  - The `Packet` trait provides a common interface for encoding and decoding network packets.
  - Packets are defined using macros (`definition_packet!`, `definition_packets!`) and are serialized into a `Vec<u8>` where the first byte is a unique packet ID, followed by the JSON-serialized payload.
  - This crate is used by both the `client` and `server` to ensure type-safe communication.

- **`extras/uid`**:
  - A small utility crate providing a `Uid` struct, which is a type-safe wrapper around `ulid::Ulid`.
  - Used for generating and handling unique identifiers throughout the application.

## 4. Development Setup

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) (install with `cargo install dioxus-cli`)
- [Node.js/Bun](https://nodejs.org/) (for Tailwind CSS compilation)

### Running the Application

1. **Install Client Dependencies**:
    Navigate to the client directory and install Node dependencies:

    ```bash
    cd client
    bun install  # or npm install
    ```

2. **Start the Backend Server**:
    Open a terminal and run the following command from the project root:

    ```bash
    cargo run --package server
    ```

    By default, the server will listen on `0.0.0.0:3000`.

3. **Start the Frontend Client**:
    Open a second terminal, navigate to the `client` directory, and use Dioxus CLI to serve the application:

    ```bash
    cd client
    dx serve
    ```

    This will compile the Dioxus application to WebAssembly and start a development server with hot-reload support, typically at `http://127.0.0.1:8080`.

4. **Access the Application**:
    Open your web browser and navigate to `http://127.0.0.1:8080`.

## 5. Communication Protocol

The client and server communicate over a WebSocket connection using a custom binary packet format defined in the `net` crate.

- **Packet Structure**: Each packet is a byte vector (`Vec<u8>`) with the following layout:
  - **Byte 0**: `u8` Packet ID. A unique identifier for the packet type.
  - **Bytes 1..N**: JSON-serialized payload of the corresponding packet struct.

- **Packet Definition**: The `net` crate uses macros to define request (`ReqPacket`) and response (`ResPacket`) packets, ensuring consistency and reducing boilerplate.

- **Example Flow**:
  1. The client sends a `Request::Login(LoginRequest { ... })` packet.
  2. The `LoginRequest` struct is serialized to JSON.
  3. The `Packet` implementation prepends the corresponding packet ID.
  4. The server receives the byte stream, reads the first byte to identify the packet type, and deserializes the rest of the payload into the appropriate struct.
  5. The server processes the request and may send one or more `Response` packets back to the client.
