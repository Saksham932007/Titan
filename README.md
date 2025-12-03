# 🚀 Titan

A production-grade, multi-threaded HTTP/1.1 web server built from scratch in Rust.

## Architecture

Titan follows a strictly modular design with separation between the library crate (`src/lib.rs`) and binary entry point (`src/main.rs`). The server implements a custom thread pool to handle concurrent connections efficiently without spawning a thread per connection.

### Directory Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # Binary entry point (thin wrapper)
├── config.rs           # Environment configuration loader
├── server.rs           # TCP server & connection handling
├── logger.rs           # Timestamped request logger
├── thread_pool.rs      # Custom ThreadPool implementation
├── website_handler.rs  # Static file serving with security
└── http/               # HTTP domain layer
    ├── mod.rs          # HTTP module + Handler trait
    ├── method.rs       # HTTP methods (GET, POST, etc.)
    ├── request.rs      # Request parsing
    ├── response.rs     # Response serialization
    └── parse_error.rs  # Parsing error types
```

## Key Features

### 1. **Idiomatic Rust**
- Proper error handling with `Result<T, E>` throughout
- No `unwrap()` in core business logic
- Type-safe HTTP method parsing with `FromStr`
- Zero-copy parsing where possible

### 2. **Thread Pool Architecture**
Instead of spawning a thread per connection (which doesn't scale), Titan uses a fixed-size thread pool:

```rust
// Worker threads share ownership of the receiver via Arc<Mutex<...>>
let receiver = Arc::new(Mutex::new(receiver));
```

Workers compete for jobs from an `mpsc` channel, implementing the **Manager-Worker** pattern.

### 3. **Security**
- **Path Traversal Protection**: Canonical path validation ensures requests like `GET /../etc/passwd` are rejected
- All file paths are validated to stay within the `public/` directory

### 4. **HTTP/1.1 Support**
- Request line parsing (`GET /path HTTP/1.1`)
- Header parsing
- Query string extraction
- UTF-8 validation

## Building & Running

### Development

```bash
# Build debug binary
cargo build

# Run the server
cargo run
```

The server reads configuration from `.env`:

```env
IP=127.0.0.1
PORT=7878
```

### Production

```bash
# Build optimized release binary
cargo build --release

# Run release binary
./target/release/titan
```

### Docker

```bash
# Build Docker image
docker build -t titan:latest .

# Run container
docker run -p 8080:8080 titan:latest
```

## Testing

Start the server and test with curl:

```bash
# Get index.html
curl http://127.0.0.1:7878/

# Test 404
curl http://127.0.0.1:7878/nonexistent.html

# Verify path traversal protection
curl http://127.0.0.1:7878/../secret.txt
```

## Design Decisions

### Why a Custom Thread Pool?

The naive approach of `thread::spawn` per connection fails under load:
- Each OS thread consumes ~2MB of stack space
- Thread creation overhead is expensive
- No backpressure mechanism

Titan's thread pool:
- Fixed resource consumption (4 workers by default)
- Lock-free job dispatch via MPSC channels
- Automatic load balancing across workers

### Why `TryFrom<&[u8]>` for Request Parsing?

Implementing `TryFrom` provides:
1. Clear error semantics (returns `Result<Request, ParseError>`)
2. Idiomatic Rust (uses the `?` operator)
3. Type safety (can't create invalid requests)

## Dependencies

```toml
[dependencies]
dotenv = "0.15"   # Environment variable loading
chrono = "0.4"    # Timestamped logging
```

Minimal dependencies keep the binary small and attack surface low.

## Commit History

This project was built iteratively with 25+ distinct commits following the Red-Green-Refactor methodology:

1. **Phase 1**: Scaffolding & configuration (commits 1-3)
2. **Phase 2**: HTTP domain types (commits 4-10)
3. **Phase 3**: Server core (commits 11-14)
4. **Phase 4**: File serving & security (commits 15-18)
5. **Phase 5**: Thread pool concurrency (commits 19-22)
6. **Phase 6**: Polish & production (commits 23-25)

## License

MIT

---

Built with ❤️ and Rust
