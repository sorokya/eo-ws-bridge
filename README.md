# EO WebSocket Bridge

![Screenshot](https://raw.githubusercontent.com/sorokya/eo-ws-bridge/refs/heads/master/screenshots/capture.png)

# EO WebSocket Bridge

**EO WebSocket Bridge** is a lightweight, cross-platform proxy that enables WebSocket clients to connect seamlessly to traditional **Endless Online** servers. It acts as a bridge between modern web-based clients and the original TCP server protocol used by EO.

This project makes it possible to play Endless Online in the browser or from environments that only support WebSocket communication.

## Features

- 🧩 **Bridges WebSocket to EO TCP protocol**
- ⚡ **Built with Rust and Tokio for fast, async performance**
- 🔁 **Bi-directional proxying of packets**
- 🖥️ **Cross-platform** (Windows, macOS, Linux)

## Getting Started

1. Clone the repo:

   ```bash
   git clone https://github.com/sorokya/eo-ws-bridge
   cd eo-ws-bridge
   ```

2. Build the project:

Make sure you have [pnpm](https://pnpm.io/) installed, then run:

   ```bash
   pnpm install
   npx tauri build
   ```

3. Run the bridge:

   ```bash
   ./src-tauri/target/release/eo-ws-bridge
   ```

By default, the bridge will listen for WebSocket clients on a configurable port (e.g. `ws://localhost:8077`) and proxy connections to an EO server (e.g. `eoserv` on port 8078).

## Example Client

Check out [sorokya/eoweb](https://github.com/sorokya/eoweb), a modern WebSocket client for Endless Online, which works seamlessly with this bridge.

## Why?

Traditional EO servers only support TCP connections, making them inaccessible to browsers or modern environments. EO WebSocket Bridge makes it possible to build web clients, mobile clients, and other creative interfaces without modifying the original EO server.

## License

MIT
