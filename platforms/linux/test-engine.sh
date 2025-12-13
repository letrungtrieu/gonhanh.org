#!/bin/bash
# Test script for GoNhanh IBus engine

set -e

echo "=== GoNhanh IBus Engine Test Script ==="
echo

# Check if binary exists
if [ ! -f "ibus-gonhanh/target/debug/ibus-engine-gonhanh" ]; then
    echo "❌ Binary not found. Run 'cargo build' first."
    exit 1
fi

echo "✅ Binary found"

# Check if IBus is running
if ! pgrep -x ibus-daemon > /dev/null; then
    echo "⚠️  IBus daemon not running. Starting..."
    ibus-daemon -drx &
    sleep 2
fi

echo "✅ IBus daemon running"

# Kill existing engine instances
pkill -f ibus-engine-gonhanh || true
sleep 1

# Start engine with debug logging
echo
echo "🚀 Starting engine with debug logs..."
echo "   (Press Ctrl+C to stop)"
echo

cd ibus-gonhanh
RUST_LOG=debug ./target/debug/ibus-engine-gonhanh
