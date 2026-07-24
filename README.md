<h1 align="center">whttp- Rust version</h1>

<p align="center">
  A minimal HTTP/1.1 server written in Rust, built for educational purposes
</p>

<p align="center">
    <img src="https://img.shields.io/badge/language-Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/protocol-HTTP%2F1.1-005C99?style=for-the-badge" alt="HTTP/1.1">
    <img src="https://img.shields.io/badge/purpose-educational-orange?style=for-the-badge" alt="Educational">
    <img src="https://img.shields.io/badge/dependencies-none-brightgreen?style=for-the-badge" alt="No dependencies">
    <img src="https://img.shields.io/badge/license-MIT-yellow?style=for-the-badge" alt="License">
</p>

---

## About

`whttp-rs` as an abbreviation is a bare-bones HTTP/1.1 server implemented from scratch in rust, ¿why rust? i will talk a lot further on my personal `YAP.md` file where i document pretty much my thoughts as the project progresses but its just i enjoy the language. 
The goal is not to build a production server. The goal is to understand what every web framework and runtime hides from you specially every API, runtime and low level "ish" program.

I want to know how a raw HTTP connection works, how requests are parsed, and how responses are constructed byte by byte. If you have ever wondered what happens between a browser sending a request and a server responding, this project answers that question in rust.

Also i am geanuately tired of the AI slop era personally.

I do use it and like to say i understand it. nonetheless its just so dumb sometimes. Also i want to use my f*ckin brain.

As before this is built as part of the Wiener Studios educational initiative. In which i basically name all my [Educational] projects as wiener-PROJECT_NAME.

---

## What You Will Learn

- Probably nothing. unless you build it. this is just me poking around.
```AI slop

- How TCP sockets work at the system call level (`socket`, `bind`, `listen`, `accept`)
- How HTTP/1.1 request parsing works — method, path, headers, body
- How to construct a valid HTTP response from scratch
- How to handle multiple client connections (blocking vs non-blocking I/O)
- Why abstractions exist — and what they cost you when you don't understand them

```

---

## Requirements

- rustc, cargo
- im running this on windows (WSL) so if this runs here  runs on yours (i think)

---

## Building

```bash
# Clone the repository
git clone https://github.com/WienerStudios/whttp.git
cd whttp

# Compile
rustc -o whttp main.c

# Or with make (if Makefile is present)
make
```

Or i think just `cargo run` works as fine as compiling it lol.


---

## Running

```bash
# Start the server on default port 8080
./whttp

```

Open a browser or use `curl` to test it:

```bash
curl -v http://localhost:8080/
```

---

## Project Structure

```text

whttp/
├── target/         # Compiled binaries and build artifacts
├── .gitignore      # Git ignore rules
├── Cargo.lock      # Exact dependency versions
├── Cargo.toml      # Rust package manifest and configuration
├── main.rs         # Entry point, TCP server loop, and HTTP logic
└── README.md       # Project documentation

```

---

## How It Works

```
Client                          whttp
  |                               |
  |--- TCP connect -------------> |  accept()
  |                               |
  |--- HTTP Request ------------> |  read() + parse
  |    GET / HTTP/1.1             |
  |    Host: localhost            |
  |                               |
  |                               |  build response
  |                               |
  |<-- HTTP Response ------------ |  write()
       HTTP/1.1 200 OK
       Content-Type: text/html
       ...
```

---

## Learning Path

This server is intentionally incomplete. The recommended approach is to build it incrementally:

1. Get a TCP socket listening and accepting connections
2. Read raw bytes from the client and print them
3. Parse the first line of the request (method and path)
4. Send a hardcoded `200 OK` response
5. Parse headers
6. Route different paths to different responses
7. Handle `POST` requests with a body
8. Experiment with keep-alive connections

Each step reveals something real about how the web works.

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<p align="center">
  <img src="https://img.shields.io/badge/Wiener%20Studios-educational-black?style=flat-square" alt="Wiener Studios">
  <img src="https://img.shields.io/badge/built%20from-scratch-lightgrey?style=flat-square" alt="Built from scratch">
</p>
