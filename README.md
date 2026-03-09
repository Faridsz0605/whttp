<h1 align="center">whttp</h1>

<p align="center">
  A minimal HTTP/1.1 server written in C, built for educational purposes
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-C-A8B9CC?style=for-the-badge&logo=c&logoColor=white" alt="C">
  <img src="https://img.shields.io/badge/protocol-HTTP%2F1.1-005C99?style=for-the-badge" alt="HTTP/1.1">
  <img src="https://img.shields.io/badge/purpose-educational-orange?style=for-the-badge" alt="Educational">
  <img src="https://img.shields.io/badge/dependencies-none-brightgreen?style=for-the-badge" alt="No dependencies">
  <img src="https://img.shields.io/badge/license-MIT-yellow?style=for-the-badge" alt="License">
</p>

---

## About

`whttp` is a bare-bones HTTP/1.1 server implemented from scratch in C. It has no external dependencies — just POSIX sockets and the standard library.

The goal is not to build a production server. The goal is to understand what every web framework and runtime hides from you: how a raw HTTP connection works, how requests are parsed, and how responses are constructed byte by byte. If you have ever wondered what happens between a browser sending a request and a server responding, this project answers that question in plain C.

Built as part of the Wiener Studios educational initiative.

---

## What You Will Learn

- How TCP sockets work at the system call level (`socket`, `bind`, `listen`, `accept`)
- How HTTP/1.1 request parsing works — method, path, headers, body
- How to construct a valid HTTP response from scratch
- How to handle multiple client connections (blocking vs non-blocking I/O)
- Why abstractions exist — and what they cost you when you don't understand them

---

## Requirements

- GCC or Clang
- POSIX-compliant OS (Linux or macOS)
- Make (optional but recommended)

No package manager. No dependencies. Just a C compiler.

---

## Building

```bash
# Clone the repository
git clone https://github.com/WienerStudios/whttp.git
cd whttp

# Compile
gcc -o whttp main.c

# Or with make (if Makefile is present)
make
```

---

## Running

```bash
# Start the server on default port 8080
./whttp

# Start on a custom port
./whttp 3000
```

Open a browser or use `curl` to test it:

```bash
curl -v http://localhost:8080/
```

---

## Project Structure

```
whttp/
├── main.c          # Entry point and server loop
├── socket.c        # TCP socket setup and connection handling
├── http.c          # HTTP request parser and response builder
├── http.h          # Shared types and function declarations
├── Makefile        # Build configuration
└── README.md
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

The server runs in a single loop: accept a connection, read the request, build a response, write it back, close the connection. No magic. No middleware. No framework.

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
