//! Test keycode translation
//!
//! Run with: cargo run --example test_keycode

use std::path::PathBuf;

// Include keycode module
#[path = "../src/keycode.rs"]
mod keycode;

fn main() {
    println!("=== GoNhanh Keycode Translation Test ===\n");

    // Test letters
    println!("Testing letters:");
    test_key(0x061, "a");
    test_key(0x041, "A");
    test_key(0x073, "s");
    test_key(0x064, "d");
    test_key(0x077, "w");

    println!("\nTesting numbers (VNI):");
    test_key(0x031, "1");
    test_key(0x032, "2");
    test_key(0x039, "9");

    println!("\nTesting special keys:");
    test_key(0x0020, "Space");
    test_key(0xFF08, "BackSpace");
    test_key(0xFF0D, "Return");

    println!("\nTesting modifiers:");
    test_modifier(0xFFE1, "Shift_L");
    test_modifier(0xFFE3, "Control_L");
    test_modifier(0xFFE9, "Alt_L");

    println!("\nTesting buffer clear keys:");
    test_buffer_clear(0x0020, "Space");
    test_buffer_clear(0xFF0D, "Return");
    test_buffer_clear(0x061, "a");

    println!("\n✅ All tests completed!");
}

fn test_key(keysym: u32, name: &str) {
    match keycode::keysym_to_macos_keycode(keysym) {
        Some(code) => println!("  {:#06x} ({:12}) → macOS keycode {:3}", keysym, name, code),
        None => println!("  {:#06x} ({:12}) → ❌ Not recognized", keysym, name),
    }
}

fn test_modifier(keysym: u32, name: &str) {
    let is_mod = keycode::is_modifier_key(keysym);
    println!(
        "  {:#06x} ({:12}) → {}",
        keysym,
        name,
        if is_mod { "✅ Modifier" } else { "❌ Not modifier" }
    );
}

fn test_buffer_clear(keysym: u32, name: &str) {
    let clears = keycode::is_buffer_clear_key(keysym);
    println!(
        "  {:#06x} ({:12}) → {}",
        keysym,
        name,
        if clears {
            "✅ Clears buffer"
        } else {
            "❌ Doesn't clear"
        }
    );
}
