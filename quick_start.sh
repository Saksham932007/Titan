#!/bin/bash
# Titan Web Server - Quick Start Script

echo "🚀 Titan Web Server - Quick Start"
echo "=================================="
echo ""

# Check if public directory exists
if [ ! -d "public" ]; then
    echo "Creating public/ directory..."
    mkdir -p public
    echo "<!DOCTYPE html><html><body><h1>Hello from Titan!</h1></body></html>" > public/index.html
fi

# Build the server
echo "Building Titan (release mode)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "To start the server:"
    echo "  ./target/release/titan"
    echo ""
    echo "Or run in development mode:"
    echo "  cargo run"
    echo ""
    echo "Server will listen on: $(grep IP .env 2>/dev/null || echo '127.0.0.1'):$(grep PORT .env 2>/dev/null | cut -d= -f2 || echo '7878')"
    echo ""
    echo "Test with:"
    echo "  curl http://127.0.0.1:7878/"
else
    echo ""
    echo "❌ Build failed. Check errors above."
    exit 1
fi
